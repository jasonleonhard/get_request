#![deny(warnings)]
// This is using the `tokio` runtime. You'll need the following dependency:
// `tokio = { version = "0.2", features = ["macros"] }`
#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let res = reqwest::get("https://hyper.rs").await?;
    let res = reqwest::get("https://jasonleonhard.com/pins/11").await?;
    // let res = reqwest::get("https://jasonleonhard.com/pins/11.json").await?;
    // println!("Status: {}", res.status());
    let body = res.text().await?;
    // println!("Body:\n\n{}", body);
    println!("{}", body);
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
