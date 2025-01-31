use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let body = reqwest::get("https://seeqdiagnostics.com")
        .await?
        .text()
        .await?;
    println!("body: {body:?}");

    Ok(())
}
