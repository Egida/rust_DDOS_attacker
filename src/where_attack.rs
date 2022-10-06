use std::io;

use crate::extra_fn::proxy_set;
use crate::ram_manger::{SAFE_PUB_VAR, UNSAFE_PUB_VAR};

pub struct AttackData {
    pub ai_mode: bool,
    pub udp_mode: bool,
}

pub fn where_attack() -> AttackData {
    let mut return_data = AttackData {
        ai_mode: false,
        udp_mode: false,
    };
    println!("Where to attack?(give url)");
    unsafe {
        get_input(&mut UNSAFE_PUB_VAR.attack_url);
        UNSAFE_PUB_VAR.attack_url = UNSAFE_PUB_VAR.attack_url.trim().to_owned();
    }
    println!("Do you want to use UDP(y or n)");
    return_data.udp_mode = true_or_no();
    if !return_data.udp_mode {
        println!("Ai Mode?(y/n), This is helpful if you have a fast pc and/or you bottle neck is your wifi!");
        return_data.ai_mode = true_or_no();

        loop {
            let mut unparsed_str = "".to_owned();
            println!("Proxy? (if you don't want one hit n)");
            get_input(&mut unparsed_str);
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
    } else {
        return_data.ai_mode = false;
    }
    loop {
        let mut unparsed_str = "".to_owned();
        println!("Threads? (if you get a dns error lower threads)");
        get_input(&mut unparsed_str);
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


fn true_or_no() -> bool {
    loop {
        let mut unparsed_str = "".to_owned();
        get_input(&mut unparsed_str);
        match unparsed_str.trim() {
            "y" => {
                return true;
            }
            "n" => {
                return false;
            }
            _ => {
                println!("please say y or n");
            }
        }
    }
}

fn get_input(input: &mut String) {
    loop {
        unsafe {
            let error = io::stdin()
                .read_line(input);
            if error.is_ok() {
                return;
            } else {
                println!("please try again");
            }
            UNSAFE_PUB_VAR.attack_url = UNSAFE_PUB_VAR.attack_url.trim().to_owned();
        }
    }
}