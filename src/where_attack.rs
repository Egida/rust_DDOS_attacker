use std::io;

use crate::extra_fn::proxy_set;
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
    println!("Ai Mode?(y/n), This is helpful if you have a fast pc and/or you bottle neck is your wifi!");
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
    loop {
        let mut unparsed_str = "".to_owned();
        println!("Proxy? (if you don't want one hit n)");
        io::stdin()
            .read_line(&mut unparsed_str)
            .expect("Failed to read input");
        match unparsed_str.trim() {
            "n" => {
                println!("{}", proxy_set("", false).expect("Failed when setting http client"));
                break;
            }
            _ => {
                let error = proxy_set(unparsed_str.trim(), true);
                match error {
                    Err(e) => println!("{}", e),
                    Ok(yay) => {
                        println!("{}", yay);
                        break;
                    }
                }
            }
        }
    }
    return_data
}

