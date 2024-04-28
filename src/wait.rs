use reqwest;
use std::error::Error;
use std::fmt;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct Wait<'a> {
    host_ports: Vec<&'a str>,
    urls: Vec<&'a str>,
    status_interval: Duration,
    check_interval: Duration,
    global_timeout: Duration,
    check_timeout: Duration,
}

impl<'a> Wait<'a> {
    pub fn new(host_ports: Vec<&'a str>, urls: Vec<&'a str>) -> Self {
        Wait {
            host_ports,
            urls,
            status_interval: Duration::from_secs(1),
            check_interval: Duration::from_millis(100),
            global_timeout: Duration::from_secs(30),
            check_timeout: Duration::from_secs(5),
        }
    }

    pub fn wait(&self) -> Result<(), Box<dyn Error>> {
        let mut checkers: Vec<Arc<Mutex<dyn Checker>>> = Vec::new();
        let mut threads = Vec::new();

        for host_port in &self.host_ports {
            let checker = HostPortChecker::new(host_port, self.check_interval, self.check_timeout)?;
            let checker_arc = Arc::new(Mutex::new(checker));
            checkers.push(checker_arc.clone());
            threads.push(thread::spawn(move || {
                checker_arc.lock().unwrap().start();
            }));
        }

        for url in &self.urls {
            let checker = UrlChecker::new(url, self.check_interval, self.check_timeout)?;
            let checker_arc = Arc::new(Mutex::new(checker));
            checkers.push(checker_arc.clone());
            threads.push(thread::spawn(move || {
                checker_arc.lock().unwrap().start();
            }));
        }

        let status_interval = self.status_interval;
        let handle = thread::spawn(move || loop {
            thread::sleep(status_interval);
            print_statuses(&checkers);
        });

        let global_timeout = self.global_timeout;
        handle.join().unwrap();
        thread::sleep(global_timeout);
        Ok(())
    }
}

fn print_statuses(checkers: &[Arc<Mutex<dyn Checker>>]) {
    for checker in checkers {
        let status = checker.lock().unwrap().status();
        println!("{}", status);
    }
}

trait Checker {
    fn status(&self) -> String;
    async fn start(&mut self);
}

struct HostPortChecker<'a> {
    host_port: &'a str,
    check_interval: Duration,
    check_timeout: Duration,
    last_status: Status,
}

impl<'a> HostPortChecker<'a> {
    fn new(
        host_port: &'a str,
        check_interval: Duration,
        check_timeout: Duration,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(HostPortChecker {
            host_port,
            check_interval,
            check_timeout,
            last_status: Status::new(host_port),
        })
    }
}

impl<'a> Checker for HostPortChecker<'a> {
    fn status(&self) -> String {
        self.last_status.to_string()
    }

    async fn start(&mut self) {
        loop {
            let start_time = Instant::now();
            match TcpStream::connect_timeout(
                &self.host_port.to_socket_addrs().unwrap().next().unwrap(),
                self.check_timeout,
            ) {
                Ok(_) => self.last_status.set(true),
                Err(_) => self.last_status.set(false),
            }
            let elapsed = start_time.elapsed();
            if elapsed < self.check_interval {
                thread::sleep(self.check_interval - elapsed);
            }
        }
    }
}

struct UrlChecker<'a> {
    url: &'a str,
    check_interval: Duration,
    check_timeout: Duration,
    last_status: Status,
}

impl<'a> UrlChecker<'a> {
    fn new(
        url: &'a str,
        check_interval: Duration,
        check_timeout: Duration,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(UrlChecker {
            url,
            check_interval,
            check_timeout,
            last_status: Status::new(url),
        })
    }
}

impl<'a> Checker for UrlChecker<'a> {
    fn status(&self) -> String {
        self.last_status.to_string()
    }

    async fn start(&mut self) {
        loop {
            let start_time = Instant::now();
            let result = reqwest::get(self.url);
            match result.await {
                Ok(response) => {
                    let status = response.status();
                    self.last_status.set(status == 200);
                }
                Err(_) => {
                    self.last_status.set(false);
                }
            }
            let elapsed = start_time.elapsed();
            if elapsed < self.check_interval {
                thread::sleep(self.check_interval - elapsed);
            }
        }
    }
}

struct Status {
    target: String,
    healthy: bool,
}

impl Status {
    fn new(target: &str) -> Self {
        Status {
            target: target.to_string(),
            healthy: false,
        }
    }

    fn set(&mut self, healthy: bool) {
        self.healthy = healthy;
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.healthy {
            write!(f, "✔️  {} -- Healthy", self.target)
        } else {
            write!(f, "❌  {} -- Unhealthy", self.target)
        }
    }
}
