use std::io;
use crate::ram_manger::{UNSAFE_PUB_VAR};

pub struct AttackData {
    pub threads: u128,
}

pub fn where_attack() -> AttackData {
    let mut return_data = AttackData {
        threads: 0,
    };
    println!("Where to attack?(give url)");
    unsafe {
        io::stdin()
            .read_line(&mut UNSAFE_PUB_VAR.attack_url)
            .expect("Failed to read input");
    }
    loop {
        let mut unparsed_str = "".to_owned();
        println!("Threads? (if you get a dns error lower threads)");
        io::stdin()
            .read_line(&mut unparsed_str)
            .expect("Failed to read input");
        match unparsed_str.trim().parse() {
            Ok(num) => {
                return_data.threads = num;
                break;
            }
            Err(e) => {
                println!("please write proper number\n (advanced error details: {})", e);
            }
        }
    }
    return_data
}