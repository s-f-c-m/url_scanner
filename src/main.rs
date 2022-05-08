use std::{env::args, time::Duration};

use rayon::prelude::*;
use reqwest::{blocking::Client, redirect};
use url_scanner::error::Error;
use url_scanner::subdomains;
use url_scanner::{model::Subdomain, ports};

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        return Err(Error::CliUsage.into());
    }

    let target = args[1].trim();

    let http_timeout = Duration::from_secs(5);
    let http_client = Client::builder()
        .redirect(redirect::Policy::limited(4))
        .timeout(http_timeout)
        .build()?;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(256)
        .build()
        .unwrap();
    pool.install(|| {
        let scan_result: Vec<Subdomain> = subdomains::enumerate(&http_client, target)
            .unwrap()
            .into_par_iter()
            .map(ports::scan_ports)
            .collect();

        for subdomain in scan_result {
            println!("{}", subdomain.domain);
            for port in subdomain.open_ports {
                println!("{}", port.port);
            }
            println!("");
        }
    });
    Ok(())
}
