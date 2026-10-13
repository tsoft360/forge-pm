use anyhow::Result;

fn version() {

}

fn install() {

}

fn check_for_updates() {

}


async fn update() -> Result<()> {
    let url = "https://api.github.com/repos/Card-Forge/forge/releases/latest";

    let client = reqwest::Client::builder()
        .user_agent("forge-pm")
        .build()?;

    let response = client
        .get(url)
        .send()
        .await?
        .error_for_status()?;

    let release: serde_json::Value = response.json().await?;

    println!("Release: {}", release["name"]);
    println!("Published: {}", release["published_at"]);

    for asset in release["assets"].as_array().into_iter().flatten() {
        println!(
            "{} - {}",
            asset["name"].as_str().unwrap_or("unknown"),
            asset["browser_download_url"]
                .as_str()
                .unwrap_or("no URL")
        );
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    update().await?;
    Ok(())
}
