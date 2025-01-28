async fn main() {
    let body = reqwest
        .get("https://seeq-diagnostics.com")
        .await?
        .text()
        .await?;
    println!("body: {body:?}");
    
}
