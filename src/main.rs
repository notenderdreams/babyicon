use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use url::Url;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = None,
    after_help = "Example:\n  babyicon \"https://hugeicons.com/icon/github?style=stroke-rounded\" -o icons/github.svg"
)]
struct Args {
    /// Icon URL from hugeicons.com  
    url: String,
    /// The output file path
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let cdn_url = build_url(&args.url)?;
    println!("CDN URL: {}", cdn_url);

    let filename = cdn_url
        .path_segments()
        .and_then(|mut segments| segments.next_back())
        .ok_or_else(|| anyhow!("Could not determine the filename from the URL"))?;

    let output_path = args.output.unwrap_or_else(|| PathBuf::from(filename));

    if let Some(parent) = output_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).context("Failed to create ouput directory")?;
        }
    }

    download_file(&cdn_url, &output_path)?;

    println!("File saved to: {}", output_path.display());

    Ok(())
}

fn build_url(input_url: &str) -> Result<Url> {
    let url = Url::parse(input_url).context("Failed to parse the URL")?;

    if url.host_str() != Some("hugeicons.com") {
        return Err(anyhow!("The URL must be from hugeicons.com"));
    }

    let segments: Vec<_> = url
        .path_segments()
        .ok_or_else(|| anyhow!("Invalid Path"))?
        .collect();

    if segments.len() < 2 || segments[0] != "icon" {
        return Err(anyhow!(
            "URL path must start with /icon/ and contain at least one more segment"
        ));
    }

    let icon_name = segments[1];
    let style = url
        .query_pairs()
        .find(|(k, _)| k == "style")
        .map(|(_, v)| v)
        .ok_or_else(|| anyhow!("Missing 'style' query parameter in the URL"))?;

    let cdn_url = format!(
        "https://cdn.hugeicons.com/icons/{}-{}.svg",
        icon_name, style
    );
    Url::parse(&cdn_url).context("Failed to construct the CDN URL")
}

fn download_file(url: &Url, output_path: &PathBuf) -> Result<()> {
    let response = reqwest::blocking::get(url.as_str()).context("Failed to send request")?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to download the file: HTTP {}",
            response.status()
        ));
    }

    let mut file = File::create(output_path).context("Failed to create file")?;
    let mut content = response;
    copy(&mut content, &mut file).context("Failed to save content to file")?;

    Ok(())
}
