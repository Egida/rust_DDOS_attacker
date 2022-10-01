use std::borrow::ToOwned;
use std::sync::Mutex;

use once_cell::sync::Lazy;

pub struct SafeGlobalVar {
    pub thread_on: f64,
    pub attack_url: String,
    pub threads_allowed: f64
}

pub static SAFE_PUB_VAR: Lazy<Mutex<SafeGlobalVar>> = Lazy::new(|| {
    Mutex::new(SafeGlobalVar {
        thread_on: 0.0,
        attack_url: "String".to_owned(),
        threads_allowed: 0.0
    })
});

pub struct UnsafePubVar {
    pub attack_url: String,
    pub amount_sent: f64,
    pub threads_on: f64,
}

pub static mut UNSAFE_PUB_VAR: Lazy<UnsafePubVar> = Lazy::new(|| {
    UnsafePubVar {
        attack_url: "".to_owned(),
        amount_sent: 0.0,
        threads_on: 0.0
    }
});
