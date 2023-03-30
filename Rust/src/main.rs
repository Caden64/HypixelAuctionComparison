mod data;
mod request;

use std::error::Error;
use tokio::task::JoinHandle;
use anyhow::{anyhow, Result};
use std::sync::mpsc::{channel};
use tokio::time::Instant;

const BASE_URL: &str = "https://api.hypixel.net/skyblock/auctions";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let client = reqwest::Client::new();
    let initial_data = request::request(&client, BASE_URL.to_string()).await?;

    let urls = request:: gen_urls(1, initial_data.total_pages as u8);

    let (tx, rx) = channel();
    let mut tasks = vec![];
    for url in urls {
        let client = client.clone();
        let tx = tx.clone();
        let task : JoinHandle<Result<()>> = tokio::spawn(async move {
            let data = request::request(&client, url).await?;
            tx.send(data).map_err(|err| anyhow!(err))?;
            Ok(())
        });
        tasks.push(task)
    }

    for task in tasks {
        task.await??;
    }

    drop(tx);

    let mut auctions = vec![];
    auctions.push(initial_data);
    for data in rx {
       auctions.push(data)
    }

    println!("{}", start.elapsed().as_secs_f32());
    println!("{}", auctions.len());

    Ok(())
}

