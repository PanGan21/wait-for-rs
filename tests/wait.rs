use std::process::Command;
use std::time::Duration;

use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;

mod utils;

#[test]
fn no_args_fail() {
    Command::cargo_bin("wait-for-rs")
        .unwrap()
        .assert()
        .failure()
        .code(1);
}

#[test]
fn invalid_urls_fails() {
    Command::cargo_bin("wait-for-rs")
        .unwrap()
        .args(&["--urls", "invalid:5000"])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn multiple_addresses_one_not_working_fails() {
    let server = utils::TestServer::new(4000, Duration::from_millis(10));

    Command::cargo_bin("wait-for-rs")
        .unwrap()
        .args(&[
            "--urls",
            "127.0.0.1:4000",
            "127.0.0.1:5000",
            "---timeout",
            "3",
        ])
        .assert()
        .failure()
        .code(1);

    drop(server);
}

#[test]
fn one_address_works() {
    let server = utils::TestServer::new(4000, Duration::from_millis(10));

    Command::cargo_bin("wait-for-rs")
        .unwrap()
        .args(&["--urls", "127.0.0.1:4000"])
        .assert()
        .code(0)
        .success();

    drop(server);
}

#[test]
fn multiple_addresses_works() {
    let servers = vec![
        utils::TestServer::new(4000, Duration::from_millis(20)),
        utils::TestServer::new(6000, Duration::from_millis(15)),
    ];

    Command::cargo_bin("wait-for-rs")
        .unwrap()
        .args(&["--urls", "127.0.0.1:4000", "127.0.0.1:6000"])
        .assert()
        .success()
        .code(0);

    drop(servers);
}

#[test]
fn multiple_addresses_with_timeout_works() {
    let server1 = utils::TestServer::new(4000, Duration::from_millis(10));
    let server2 = utils::TestServer::new(6000, Duration::from_millis(10));

    Command::cargo_bin("wait-for-rs")
        .unwrap()
        .args(&[
            "--urls",
            "127.0.0.1:4000",
            "127.0.0.1:6000",
            "--timeout",
            "5",
        ])
        .assert()
        .success();

    drop(server1);
    drop(server2);
}

#[test]
fn one_address_one_https_works() {
    let server1 = utils::TestServer::new(4000, Duration::from_millis(10));

    Command::cargo_bin("wait-for-rs")
        .unwrap()
        .args(&["--urls", "127.0.0.1:4000", "https://google.com"])
        .assert()
        .success();

    drop(server1);
}
