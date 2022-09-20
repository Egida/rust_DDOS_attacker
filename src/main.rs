mod attack;
#[tokio::main]
async fn main() {
    attack::start().await;
}

pub static ATTACK_URL: &str = "https://carghaiapi.herokuapp.com";
pub static FORCE: u128 = 2500;
// 2000 to can take down a replit flask api
// This is the code
// app.route('/hard')
// def hard_dos():
//   return "failure"
