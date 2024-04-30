use log::LevelFilter;
use std::{error::Error, process::exit};
use structopt::StructOpt;

use wait_for_rs::WaitService;

#[derive(Debug, StructOpt)]
#[structopt(name = "url-wait", about = "A command-line utility to wait for URLs.")]
struct Opt {
    #[structopt(short, long)]
    urls: Vec<String>,

    #[structopt(short, long, default_value = "10")]
    timeout: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    let opt = Opt::from_args();

    let wait = WaitService::new(opt.urls, opt.timeout)?;

    let result = wait.wait_for_services();
    let exit_code: i32 = if result.is_ok() { 0 } else { 1 };

    exit(exit_code);
}
