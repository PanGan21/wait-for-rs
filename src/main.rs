use std::error::Error;
use structopt::StructOpt;

mod wait;

#[derive(Debug, StructOpt)]
#[structopt(name = "url-wait", about = "A command-line utility to wait for URLs.")]
struct Opt {
    #[structopt(short, long)]
    urls: Vec<String>,

    #[structopt(short, long, default_value = "1")]
    check_interval: u64,

    #[structopt(short, long, default_value = "30")]
    timeout: u64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    wait::wait_for_urls(opt.urls, opt.check_interval, opt.timeout).await?;
    Ok(())
}
