use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut args = env::args();

    let prog = args.next().unwrap();
    let input = match args.next() {
        Some(v) => v,
        None => {
            eprintln!("Usage: {} <url> [output]", prog);
            return;
        }
    };

    let output = args.next();

    let cdn_url = match build_url(&input) {
        Some(u) => u,
        None => {
            eprintln!("Invalid URL");
            return;
        }
    };

    let filename = cdn_url.rsplit('/').next().unwrap();
    let output_path = output.unwrap_or_else(|| filename.to_string());

    if let Some(parent) = Path::new(&output_path).parent() {
        std::fs::create_dir_all(parent).ok();
    }

    let status = Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(&output_path)
        .arg(&cdn_url)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("Saved to {}", output_path);
        }
        _ => {
            eprintln!("Download failed");
        }
    }
}

fn build_url(input: &str) -> Option<String> {
    if !input.starts_with("https://hugeicons.com/icon/") {
        return None;
    }

    let (path, query) = input.split_once('?')?;

    let icon = path
        .trim_start_matches("https://hugeicons.com/icon/")
        .split('/')
        .next()?;

    let style = query
        .split('&')
        .find_map(|p| {
            let (k, v) = p.split_once('=')?;
            if k == "style" { Some(v) } else { None }
        })?;

    Some(format!(
        "https://cdn.hugeicons.com/icons/{}-{}.svg",
        icon, style
    ))
}