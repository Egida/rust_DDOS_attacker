use std::io;

use crate::ram_manger::{SAFE_PUB_VAR, UNSAFE_PUB_VAR};

pub struct AttackData {
    pub ai_mode: bool,
}

pub fn where_attack() -> AttackData {
    let mut return_data = AttackData {
        ai_mode: false,
    };
    println!("Where to attack?(give url)");
    unsafe {
        io::stdin()
            .read_line(&mut UNSAFE_PUB_VAR.attack_url)
            .expect("Failed to read input");
    }
    println!("Ai Mode?(y/n), Warning DDOS ai is very unstable and may not work as expected!");
    loop {
        let mut unparsed_str = "".to_owned();
        io::stdin()
            .read_line(&mut unparsed_str)
            .expect("Failed to read input");
        match unparsed_str.trim() {
            "y" => {
                return_data.ai_mode = true;
                break;
            }
            "n" => {
                return_data.ai_mode = false;
                break;
            }
            _ => {
                println!("please say y or n");
            }
        }
    }
    loop {
        let mut unparsed_str = "".to_owned();
        println!("Threads? (if you get a dns error lower threads)");
        io::stdin()
            .read_line(&mut unparsed_str)
            .expect("Failed to read input");
        match unparsed_str.trim().parse() {
            Ok(num) => {
                SAFE_PUB_VAR.lock().expect("failure when parsing").threads_allowed = num;
                break;
            }
            Err(e) => {
                println!("please write proper number\n (advanced error details: {})", e);
            }
        }
    }
    return_data
}

