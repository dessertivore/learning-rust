use reqwest::{self};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Call colour endpoint and print response to console
    let resp = reqwest::get("http://localhost:8000/colour").await?;
    let body=resp.text().await?;
    println!("Response: {}", body);
    Ok(())
}