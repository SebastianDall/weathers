







pub fn forecast(args: ForecastArgs) -> Result<()> {
    let body = reqwest::get("https://google.com")
        .await?
        .text()
        .await?;
    println!("body: {body:?}");

    Ok(())
    
}
