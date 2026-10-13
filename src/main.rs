use anyhow::Result;
use serde::Deserialize;
use std::path::Path;
use std::fs;

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}


async fn download_file(
    url: &str,
    output: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(url)
        .header("User-Agent", "forge-pm")
        .send()
        .await?
        .error_for_status()?;

    let bytes = response.bytes().await?;

    fs::write(output, &bytes);

    println!("Downloaded {} bytes to {:?}", bytes.len(), output);

    Ok(())
}

async fn update() -> Result<()> {
    // let url = "https://api.github.com/repos/Card-Forge/forge/releases/latest";

    let release: Release = reqwest::Client::new()
        .get("https://api.github.com/repos/Card-Forge/forge/releases/latest")
        .header("User-Agent", "forge-pm")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    for asset in release.assets {
        if asset.name == "forge-installer-2.0.15.tar.bz2" {
            download_file(&asset.browser_download_url.to_string(), Path::new("forge.tar.bz2")).await;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    update().await?;
    Ok(())
}
