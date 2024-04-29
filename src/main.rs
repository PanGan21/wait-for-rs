use log::LevelFilter;
use std::{
    error::Error,
    process::exit,
    thread::{spawn, JoinHandle},
};
use structopt::StructOpt;

mod wait;

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

    let mut success = true;
    let mut threads: Vec<JoinHandle<bool>> = Vec::new();
    for url in opt.urls {
        let thread = spawn(move || wait::wait_for_service(&url, opt.timeout).is_ok());
        threads.push(thread);
    }

    for thread in threads {
        success &= thread.join().unwrap_or(false);
    }

    let exit_code: i32 = if success { 0 } else { 1 };
    exit(exit_code);
}
