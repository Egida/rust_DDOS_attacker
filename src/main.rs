mod attack;
#[tokio::main]
async fn main() {
    attack::start().await;
}
