use crate::data;
use anyhow::{Result};

pub fn gen_urls(start: u8, end: u8) -> Vec<String> {
    let mut urls = vec![];
    for i in start..end {
        urls.push(crate::BASE_URL.to_string() + "?page=" + &*i.to_string())
    }
    urls
}

pub async fn request(client: &reqwest::Client, url: String) -> Result<data::Auctions>{
    let res = client.get(url).send().await?;
    let json: data::Auctions = res.json().await?;
    Ok(json)
}
