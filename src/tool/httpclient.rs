use std::collections::HashMap;

pub async fn httpClient() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?;
    println!("{:#?}", resp);
    Ok(())
}