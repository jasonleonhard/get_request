#![deny(warnings)]
#![warn(rust_2018_idioms)]
use hyper::{body::HttpBody as _, Client};
use std::env;
use tokio::io::{self, AsyncWriteExt as _};

// A simple type alias so as to DRY.
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
// async fn main() -> Result<()> {
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
    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    let url = url.parse::<hyper::Uri>().unwrap();
    // optionally: require url to use http or https at minimum
    if url.scheme_str() == Some("http") || url.scheme_str() == Some("https") {
        let res = reqwest::get(query).await?;
        let body = res.text().await?;
        println!("{}", body);
        // note: we must not keep extra println! if we wish to feed our code to | jq
        // println!("Status: {}", res.status());
        // println!("Body:\n\n{}", body);
        // println!("This example only works with 'http' URLs.");
        return Ok(());
    }
    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();
    let mut res = client.get(url).await?;
    // note: we must not keep extra println! if we wish to feed our code to | jq
    // println!("Response: {}", res.status());
    // println!("Headers: {:#?}\n", res.headers());
    // Stream the body, writing each chunk to stdout as we get it
    // (instead of buffering and printing at the end).
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }
    // note: we must not keep extra println! if we wish to feed our code to | jq
    // println!("\n\nDone!");
    Ok(())
}

// The [cfg(not(target_arch = "wasm32"))] above prevent building the tokio::main function
// for wasm32 target, because tokio isn't compatible with wasm32.
// If you aren't building for wasm32, you don't need that line.
// The two lines below avoid the "'main' function not found" error when building for wasm32 target.
#[cfg(target_arch = "wasm32")]
fn main() {}

// RUN: this and get json formatted correctly currently does not take url
// cargo run | jq
// RESULT_BELOW:
// {
//     "id": 11,
//     "title": "https://pleasegrab.com",
//     "description": "An intelligent place to discover, organize and analyze exceptional food outings",
//     "created_at": "2020-07-08T23:38:49.773Z",
//     "updated_at": "2020-07-08T23:38:49.773Z",
//     "image_file_name": "pleasegrab.png",
//     "image_content_type": "image/png"
// }

// but soon we want to pass a url like so
// cargo run "https://jasonleonhard.com/pins/11" | jq

// example: https://github.com/seanmonstar/reqwest
// ref: https://github.com/seanmonstar/reqwest/blob/master/Cargo.toml
// notice of the libraries used here they include hyper... reqwest is a easier way to use hyper
