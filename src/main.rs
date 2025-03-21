#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_dynamodb::Client::new(&config);
    
    println!("Region: {:?}", client.config().region());
    
    println!("Hello, world!");
}
