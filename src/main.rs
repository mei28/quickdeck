use regex::Regex;
use reqwest;
use tokio;
use webbrowser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <slideshare-url>", args[0]);
        std::process::exit(1);
    }
    let url = &args[1];

    let html = reqwest::get(url).await?.text().await?;

    let embed_url = extract_embed_url(&html)?;
    println!("Embed URL: {}", embed_url);

    // ここでブラウザを開く
    if webbrowser::open(&embed_url).is_ok() {
        println!("Slide opened in web browser.");
    } else {
        eprintln!("Failed to open web browser.");
    }

    Ok(())
}

fn extract_embed_url(html: &str) -> Result<String, String> {
    let re = Regex::new(r"https://www\.slideshare\.net/slideshow/embed_code/key/\w+")
        .map_err(|e| e.to_string())?;
    let mut captures = re.captures_iter(html);
    if let Some(cap) = captures.next() {
        Ok(cap[0].to_string())
    } else {
        Err("No embed URL found".to_string())
    }
}
