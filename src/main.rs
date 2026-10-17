use serde::Deserialize;
use anyhow::{Context, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use clap::{Parser, Subcommand};
use std::{
    path::{Path, PathBuf},
    time::Duration,
    process::Command,
};
use bzip2::read::BzDecoder;
use tar::Archive;
use tokio::{
    fs::File,
    io::AsyncWriteExt,
};

#[derive(Parser)]
#[command(name = "forge-pm", version, about = "A packet manager for forge-desktop")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Update {
        #[arg(short, long, default_value = "forge.tar.bz2")]
        output: PathBuf,
        #[arg(long)]
        check: bool,
    },

    Install {
        #[arg(short, long)]
        directory: Option<PathBuf>,
    },

    Unpack,
} 

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

async fn extract_tar_bz2(path: &str) -> Result<()> {
    let p = Path::new("/home/dragon/forge");
    if !p.exists() {
        Command::new("mkdir").arg("/home/dragon/forge").spawn();
    }
    let progress = ProgressBar::new_spinner();
    progress.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg} unpacking forge")?
    );
    progress.enable_steady_tick(Duration::from_millis(100));
    Command::new("tar")
        .args(["-xf", path, "-C", "/home/dragon/forge"])
        .spawn()?
        .wait();
    
    Ok(())
}

async fn download_file(url: &str, output: &Path) -> Result<()> {
    let client = reqwest::Client::builder()
        .user_agent("forge-pm")
        .build()?;

    let response = client
        .get(url)
        .send()
        .await?
        .error_for_status()?;

    let total_size = response.content_length();

    let progress = match total_size {
        Some(size) => ProgressBar::new(size),
        None => ProgressBar::new_spinner(),
    };

    progress.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, ETA {eta})"
        )?
        .progress_chars("=>-"),
    );

    progress.enable_steady_tick(Duration::from_millis(100));

    let mut file = File::create(output)
        .await
        .with_context(|| format!("Cannot create {}", output.display()))?;

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
        progress.inc(chunk.len() as u64);
    }

    file.flush().await?;
    progress.finish_with_message("Download complete");

    Ok(())
}


async fn download() -> Result<()> {
    // let url = "https://api.github.com/repos/Card-Forge/forge/releases/latest";

    let release: Release = reqwest::Client::new()
        .get("https://api.github.com/repos/Card-Forge/forge/releases/latest")
        .header("User-Agent", "forge-pm")
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Command::new("rm").arg("forge.tar.bz2").spawn(); 
    for asset in release.assets {
        if asset.name.starts_with("forge-installer") && asset.name.ends_with(".tar.bz2") {
            download_file(&asset.browser_download_url.to_string(), Path::new("forge.tar.bz2")).await;
        }
    }

    Ok(())
}

async fn check_version() -> Result<()> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Update { output, check } => {
            println!("Updating Forge...");
            println!("Output: {}", output.display());
            println!("Check if file exists: {check}");
            download().await;
            extract_tar_bz2("forge.tar.bz2").await;
            Ok(())
        }

        Commands::Install { directory } => {
            let directory = directory.unwrap_or_else(|| {
                PathBuf::from("./forge.tar.bz2")
            });

            println!("Installing Forge to {}", directory.display());
            Ok(())
        }

        Commands::Unpack => {
            extract_tar_bz2("forge.tar.bz2").await;
            Ok(())
        }
    }
}