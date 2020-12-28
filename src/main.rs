// asynchronous example of tokio reqwest
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
// OUTPUT BELOW is also same as link above
// {
//     "origin": "37.120.149.92",
// }

// ref: https://github.com/seanmonstar/reqwest
