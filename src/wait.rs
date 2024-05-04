use crate::{Result, WaitServiceError};
use log::{debug, error, info};
use std::{
    collections::HashMap,
    io,
    net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs},
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

/// The default interval in milliseconds between pings for the same url
const DEFAULT_INTERVAL: u64 = 500;
/// The default connection timeout in seconds
const DEAFULT_CONNECTION_TIMEOUT: u64 = 1;

pub struct WaitService {
    addresses: HashMap<SocketAddr, String>,
    timeout: Duration,
    interval: u64,
}

impl WaitService {
    pub fn new(urls: Vec<String>, timeout_seconds: u64) -> Result<WaitService> {
        if urls.is_empty() {
            return Err(WaitServiceError::UrlsEmpty);
        }

        let timeout = if timeout_seconds == 0 {
            Duration::MAX
        } else {
            Duration::from_secs(timeout_seconds)
        };

        let addresses_result: Result<HashMap<SocketAddr, String>> = urls
            .into_iter()
            .map(|url| resolve_address(&url).map(|addr| (addr, url.clone())))
            .collect();

        Ok(WaitService {
            addresses: addresses_result?,
            timeout,
            interval: DEFAULT_INTERVAL,
        })
    }

    pub fn wait_for_services(self) -> Result<()> {
        let mut threads = Vec::new();

        for addr in self.addresses {
            let thread = spawn(move || {
                wait_for_tcp_socket(addr.clone(), self.timeout.clone(), self.interval.clone())
            });
            threads.push(thread);
        }

        for thread in threads {
            thread
                .join()
                .unwrap_or(Err(WaitServiceError::ServiceNotAvailable))?;
        }

        Ok(())
    }
}

fn resolve_address(url: &str) -> Result<SocketAddr> {
    match url.to_socket_addrs() {
        Ok(mut addr) => {
            return Ok(addr.next().unwrap());
        }
        Err(e) => return Err(WaitServiceError::Io(e)),
    }
}

fn wait_for_tcp_socket(
    address_set: (SocketAddr, String),
    timeout: Duration,
    interval_millis: u64,
) -> Result<()> {
    let timer = Instant::now();
    let url = address_set.1;
    loop {
        debug!("Ping {url}");

        let timeout_left = timeout.saturating_sub(timer.elapsed());
        if timeout_left.is_zero() {
            let error = io::Error::new(io::ErrorKind::TimedOut, "Time is up");
            return Err(WaitServiceError::Io(error));
        }

        match TcpStream::connect_timeout(
            &address_set.0,
            Duration::from_secs(DEAFULT_CONNECTION_TIMEOUT),
        ) {
            Ok(connection) => {
                let _ = connection.shutdown(Shutdown::Both);
                let duration = timer.elapsed().as_secs_f32().max(0.1);
                info!("{url} is available after {duration:.1} seconds.");
                return Ok(());
            }
            Err(error) => {
                if timer.elapsed() >= timeout {
                    error!(
                        "{url} timed out after waiting for {:#?} seconds ({error}).",
                        timeout
                    );
                    return Err(WaitServiceError::Io(error));
                }
            }
        }
        sleep(Duration::from_millis(interval_millis));
    }
}
