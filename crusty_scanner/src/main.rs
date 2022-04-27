// This is sample code from the book "black hat rust" by Sylvain
// Kerkour.



use futures::{stream, StreamExt};
use reqwest::Client;
use std::{env, time::{Duration, Instant} };

mod error;
mod model;
mod ports;
mod subdomains;
mod common_ports;

pub use error::Error;
use model::Subdomain;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 || &args[1] != "crusty_scan" {
        return Err(Error::CliUsage.into());
    }

    let target = args[2].as_str();

    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;

    let ports_concurrency = 200;
    let subdomains_concurrency = 100;
    let scan_start = Instant::now();

    let subdomains = subdomains::enumerate(&http_client, target).await?;

    // Using buffer_unordered + collect
    let scan_results: Vec<Subdomain> = stream::iter(subdomains.into_iter())
        .map(|subdomain| ports::scan_ports(ports_concurrency, subdomain))
        .buffer_unordered(subdomains_concurrency)
        .collect()
        .await;

    // _____Using an Arc<Mutex<T>>_____
    // let res: Arc<Mutex<Vec<Subdomain>>> = Arc::new(Mutex::new(Vec::new));

    // stream::iter(subdomains.into_iter())
    //    .for_each_concurrent(subdomains_concurrency, |subdomain| {
    //        let res = res.clone();
    //        async move {
    //            let subdomain = ports::scan_ports(ports_concurrency, subdomain).await;
    //            res.lock.await.push(subdomain)
    //        }
    //    })
    //    .await;

    let scan_duration = scan_start.elapsed();
    println!("Scan completed in {:?}", scan_duration);

    for subdomain in scan_results {
        println!{"{}:", &subdomain.domain};
        for port in &subdomain.open_ports {
            println!("     {}: open", port.port);
        }

        println!("");
    }

    Ok(())
}