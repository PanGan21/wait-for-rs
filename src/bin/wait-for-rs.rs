use log::LevelFilter;
use std::process::exit;
use structopt::StructOpt;

use wait_for_rs::{Result, WaitService};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "wait-for-rs",
    about = "A command-line utility to wait for URLs."
)]
struct Opt {
    #[structopt(short, long)]
    urls: Vec<String>,

    #[structopt(short, long, default_value = "10")]
    timeout: u64,
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let opt = Opt::from_args();

    if let Err(err) = run(opt) {
        eprintln!("{}", err);
        exit(1);
    }
}

fn run(opt: Opt) -> Result<()> {
    let wait = WaitService::new(opt.urls, opt.timeout)?;

    wait.wait_for_services()?;

    Ok(())
}
