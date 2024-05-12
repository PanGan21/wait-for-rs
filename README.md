# wait-for-rs

A versatile Rust library and command-line tool for waiting on network resources to become available. It can be seamlessly integrated into Rust projects as a library or used as a standalone command-line utility. Additionally, it can be easily incorporated into Dockerfiles for containerized environments.

## Features

- <b>Flexible</b>: It can be used as both a library and a command-line tool, offering flexibility in integration and usage.
- <b>Easy Installation</b>: The command-line tool can be installed globally and used directly in shell scripts or Dockerfiles.
- <b>Network Resource Waiting</b>: Use `wait-for-rs` to wait for network resources such as URLs and TCP sockets to become available.
- <b>Customizable</b>: Configure wait durations, timeouts, and intervals according to your specific requirements.

### Installation

To install wait-for-rs as a command-line tool, simply run:

```bash
cargo install wait-for-rs
```

### Library

To use wait-for-rs as a library in your Rust project, add the following to your Cargo.toml:

```toml
[dependencies]
wait-for-rs = { git = "https://github.com/PanGan21/wait-for-rs.git", branch = "main" }
```

### Usage

#### Command-Line Tool

To start the server:

```bash
# Wait for a URL to become available
wait-for-rs http://example.com --timeout 30

# Wait for a TCP socket to become available
wait-for-rs 127.0.0.1:8080 --timeout 60
```

#### Library

```rust
use wait_for_rs::{WaitService, Result};

fn main() -> Result<()> {
    // Create a WaitService instance and wait for services
    let wait_service = WaitService::new(vec!["http://example.com".to_string()], 30)?;
    wait_service.wait_for_services()?;
    Ok(())
}
```

For more detailed usage instructions and API documentation, please refer to the [examples](./examples/).

##### Run the tests

```rust
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
