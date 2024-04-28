use futures::future;
use reqwest::Client;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

pub async fn wait_for_urls(
    urls: Vec<String>,
    check_interval: u64,
    timeout: u64,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let start_time = std::time::Instant::now();

    let mut all_urls_available = false;

    while start_time.elapsed().as_secs() < timeout && !all_urls_available {
        let mut futures = Vec::new();

        for url in &urls {
            let client = client.clone();
            let future = async move {
                match client.get(url).send().await {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            println!("{} is available!", url);
                            return true;
                        }
                    }
                    Err(_) => {}
                }
                false
            };
            futures.push(future);
        }

        let results = future::join_all(futures).await;
        all_urls_available = results.iter().all(|&r| r);

        if !all_urls_available {
            sleep(Duration::from_secs(check_interval)).await;
        }
    }

    if all_urls_available {
        println!("All URLs are available!");
    } else {
        println!("Timeout reached, some URLs are not available.");
    }

    Ok(())
}
