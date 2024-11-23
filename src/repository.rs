use clap::builder::Str;
use reqwest::Error;
use serde::Deserialize;
use crate::global::read_global_toml;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}
pub async fn get_latest_version(ver: String) -> Result<(), Error> {
    let owner = "hacimertgokhan";
    let repo = "denis";
    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "RustApp")
        .send()
        .await?
        .json::<Release>()
        .await?;

    println!("Current denis-cli version: {}", ver);
    println!("Latest denis-cli version: {}", response.tag_name);
    Ok(())
}