use regex::Regex;
use reqwest;
use tokio;
use webbrowser;

async fn run_app(url: &str) -> Result<(), String> {
    let html = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())?;
    let embed_url = extract_embed_url(&html)?;
    if webbrowser::open(&embed_url).is_err() {
        return Err("Failed to open web browser.".to_string());
    }
    println!("Slide opened in web browser: {}", embed_url);
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <slideshare-url>", args[0]);
        std::process::exit(1);
    }
    let url = &args[1];
    run_app(url).await.map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run_app_with_valid_args() {
        let url = "https://www.slideshare.net/iTooooooooooooT/itolab-how-to-survey-2017";
        assert!(run_app(url).await.is_ok());
    }

    #[tokio::test]
    async fn test_run_app_with_invalid_args() {
        let url = "https://www.slideshare.net/mei28/notty";
        assert!(run_app(url).await.is_err());
    }

    #[test]
    fn test_extract_embed_url_valid() {
        let html = r#"
"iframeEmbed":{"url":"https://www.slideshare.net/slideshow/embed_code/key/6ab6KSKYN23bGZ","height":486,"width":597},
        "#;
        let result = extract_embed_url(html);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "https://www.slideshare.net/slideshow/embed_code/key/6ab6KSKYN23bGZ"
        );
    }

    #[test]
    fn test_extract_embed_url_invalid() {
        let html = "<html></html>";
        let result = extract_embed_url(html);
        assert!(result.is_err());
    }
}
