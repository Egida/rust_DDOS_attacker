use std::sync::MutexGuard;

use tokio::time::Instant;

use crate::extra_fn;
use crate::ram_manger::{SAFE_PUB_VAR, SafeGlobalVar, UNSAFE_PUB_VAR};

pub async fn start() {
    tokio::spawn(async {
        extra_fn::time_function()
    });
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
                    let error_data = extra_fn::request();
                    match error_data.await {
                        Ok(status_code) => {
                            UNSAFE_PUB_VAR.amount_sent += 1.0;
                            UNSAFE_PUB_VAR.threads_on -= 1.0;
                            if now.elapsed().as_secs() > 40 {
                                let wait = subtract();
                                println!(
                                    "Threads on {},\n Status code {},\n Request sent per 10 mil {}\n Time Elapsed {}",
                                    UNSAFE_PUB_VAR.threads_on,
                                    status_code.status(),
                                    UNSAFE_PUB_VAR.amount_sent,
                                    now.elapsed().as_secs()
                                );
                                wait.await;
                            } else {
                                let wait = add();
                                println!(
                                    "Threads on {},\n Status code {},\n Request sent per 10 mil {}\n Time Elapsed {}",
                                    UNSAFE_PUB_VAR.threads_on,
                                    status_code.status(),
                                    UNSAFE_PUB_VAR.amount_sent,
                                    now.elapsed().as_secs()
                                );
                                wait.await;
                            }
                        }
                        Err(data) => {
                            let wait = subtract();
                            println!(
                                "Threads on {}, Status ERROR {}\n  Request sent per 10 mil {}\n, Time Elapsed {}",
                                UNSAFE_PUB_VAR.threads_on,
                                data,
                                UNSAFE_PUB_VAR.amount_sent,
                                now.elapsed().as_secs()
                            );
                            UNSAFE_PUB_VAR.threads_on -= 1.0;
                            wait.await;
                        }
                    }
                }
            });
        }
    }
}


fn get_pub_var() -> MutexGuard<'static, SafeGlobalVar> {
    loop {
        if let Ok(data) = SAFE_PUB_VAR.lock() {
            return data;
        }
        println!("Waiting For Unlock Of SafeGlobalVar")
    }
}

async fn add() {
    let mut data = get_pub_var();
    data.threads_allowed += 0.05;
    data.thread_on -= 1.0;
    println!("changing threads to: {}", &data.threads_allowed);
}

async fn subtract() {
    let mut data = get_pub_var();
    data.threads_allowed -= 0.5;
    data.thread_on -= 1.0;
    println!("changing threads to: {}", &data.threads_allowed);
}

