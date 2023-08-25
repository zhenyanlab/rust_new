use std::collections::HashMap;

pub async fn httpClient() -> Result<String, Box<dyn std::error::Error>> {
    println!("httpClient-start");
    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;
    println!("body = {:?}", body);
    Ok(body)
}