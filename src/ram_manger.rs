use std::borrow::ToOwned;
use std::sync::Mutex;

use once_cell::sync::Lazy;

pub struct SafeGlobalVar {
    pub thread_on: u128,
    pub attack_url: String,
}

pub static SAFE_PUB_VAR: Lazy<Mutex<SafeGlobalVar>> = Lazy::new(|| {
    Mutex::new(SafeGlobalVar {
        thread_on: 0,
        attack_url: "String".to_owned(),
    })
});

pub struct UnsafePubVar {
    pub attack_url: String,
    pub amount_sent: u128,
    pub threads_on: u128,
}

pub static mut UNSAFEPUBVAR: Lazy<UnsafePubVar> = Lazy::new(|| {
    UnsafePubVar {
        attack_url: "".to_owned(),
        amount_sent: 0,
        threads_on: 0,
    }
});
