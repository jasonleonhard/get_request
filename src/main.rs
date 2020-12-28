#![deny(warnings)]
#![warn(rust_2018_idioms)]
use hyper::{body::HttpBody as _, Client};
use std::env;
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init(); // basic logger

    // Some simple CLI args requirements...
    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    // HTTPS requires picking a TLS implementation, so give a better
    // warning if the user tries to request an 'https' URL.
    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() != Some("http") {
        println!("This example only works with 'http' URLs.");
        return Ok(());
    }

    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();
    let mut res = client.get(url).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }

    println!("\n\nDone!");

    Ok(())
}

// run without logger
// cargo run "http://httpbin.org/get"
// OUTPUT will be:
// Response: 200 OK
// Headers: {
//     "date": "Mon, 28 Dec 2020 04:33:42 GMT",
//     "content-type": "application/json",
//     "content-length": "198",
//     "connection": "keep-alive",
//     "server": "gunicorn/19.9.0",
//     "access-control-allow-origin": "*",
//     "access-control-allow-credentials": "true",
// }
// Done!
// {
//   "args": {},
//   "headers": {
//     "Host": "httpbin.org",
//     "X-Amzn-Trace-Id": "Root=1-5fe96026-57d6c17516926c7b4ef5f476"
//   },
//   "origin": "37.120.149.92",
//   "url": "http://httpbin.org/get"
// }

// or with logger
// RUST_LOG=trace cargo run "http://httpbin.org/get"
// OUTPUT will be:
////////// another example
// https://jasonleonhard.com/pins/11.json
// Response: 301 Moved Permanently
// Headers: {
//     "date": "Mon, 28 Dec 2020 04:43:20 GMT",
//     "transfer-encoding": "chunked",
//     "connection": "keep-alive",
//     "cache-control": "max-age=3600",
//     "expires": "Mon, 28 Dec 2020 05:43:20 GMT",
//     "location": "https://jasonleonhard.com/pins/11.json",
//     "cf-request-id": "0749417e4b00000d3ea338e000000001",
//     "report-to": "{\"endpoints\":[{\"url\":\"https:\/\/a.nel.cloudflare.com\/report?s=RQfN0aQ1tHVCsZ3f3hjqxGTVHmUWQhp9mvPfdoXkw6si0e3JVB8iuKKx%2BO5NRA5b6NkZrYM%2B1qpopBo3Ux4t8ET%2B4G047jDPSwEk1uplbEAWTA%3D%3D\"}],\"group\":\"cf-nel\",\"max_age\":604800}",
//     "nel": "{\"report_to\":\"cf-nel\",\"max_age\":604800}",
//     "x-content-type-options": "nosniff",
//     "server": "cloudflare",
//     "cf-ray": "60889eaa199f0d3e-ARN",
// }
// Done!
