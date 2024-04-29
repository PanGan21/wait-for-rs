use log::{debug, error, info};
use std::{
    io,
    net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs},
    thread::sleep,
    time::{Duration, Instant},
};

fn resolve_address(url: &str) -> Result<SocketAddr, std::io::Error> {
    match url.to_socket_addrs() {
        Ok(mut addr) => {
            return Ok(addr.next().unwrap());
        }
        Err(e) => return Err(e),
    }
}

fn wait_for_tcp_socket(url: &str, timeout: Duration) -> Result<(), std::io::Error> {
    let timer = Instant::now();
    let address = resolve_address(url)?;
    loop {
        debug!("Ping {url}");
        let timeout_left = timeout.saturating_sub(timer.elapsed());
        if timeout_left.is_zero() {
            let error = io::Error::new(io::ErrorKind::TimedOut, "Time is up");
            return Err(error);
        }

        // TODO: Use separate var for this timeout
        match TcpStream::connect_timeout(&address, Duration::from_secs(1)) {
            Ok(connection) => {
                let _ = connection.shutdown(Shutdown::Both);
                debug!("{url} available!");
                return Ok(());
            }
            Err(error) => {
                if timer.elapsed() >= timeout {
                    debug!("{url} not available!");
                    return Err(error);
                }
            }
        }
        sleep(Duration::from_millis(500));
    }
}

pub fn wait_for_service(url: &str, timeout_seconds: u64) -> Result<(), std::io::Error> {
    let timer = Instant::now();

    info!("Waiting {timeout_seconds} seconds for {url}...");

    let timeout = if timeout_seconds == 0 {
        Duration::MAX
    } else {
        Duration::from_secs(timeout_seconds)
    };

    let connect_result = wait_for_tcp_socket(url, timeout);

    match connect_result {
        Ok(_) => {
            let duration = timer.elapsed().as_secs_f32().max(0.1);
            info!("{url} is available after {duration:.1} seconds.");
        }
        Err(ref error) => {
            error!("{url} timed out after waiting for {timeout_seconds} seconds ({error}).");
        }
    }

    connect_result
}
