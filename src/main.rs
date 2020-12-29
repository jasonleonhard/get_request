#![deny(warnings)]
#![warn(rust_2018_idioms)]
use hyper::{body::HttpBody as _, Client};
use std::env;
use tokio::io::{self, AsyncWriteExt as _};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let url = match env::args().nth(1) {
        Some(url) => url,
        None => {
            println!("Usage: client <url>");
            return Ok(());
        }
    };

    let args: Vec<String> = env::args().collect();
    let query = &args[1];

    let url = url.parse::<hyper::Uri>().unwrap();
    if url.scheme_str() == Some("http") || url.scheme_str() == Some("https") {
        let res = reqwest::get(query).await?;
        let body = res.text().await?;
        println!("{}", body);
        return Ok(());
    }
    fetch_url(url).await
}

async fn fetch_url(url: hyper::Uri) -> Result<()> {
    let client = Client::new();
    let mut res = client.get(url).await?;
    while let Some(next) = res.data().await {
        let chunk = next?;
        io::stdout().write_all(&chunk).await?;
    }
    Ok(())
}
