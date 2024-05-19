use wait_for_rs::WaitService;

fn main() {
    // let urls = ["google.com:443".to_string(), "github.com:443".to_string()].to_vec();
    let urls = ["google.com:443".to_string()].to_vec();
    let timeout = 10;

    let wait_service = WaitService::new(urls, timeout).unwrap();
    wait_service.wait_for_services().unwrap();
}
