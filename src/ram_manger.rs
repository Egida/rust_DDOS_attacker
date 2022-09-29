use std::borrow::ToOwned;
use std::sync::Mutex;

use once_cell::sync::Lazy;

pub struct GlobalVar {
    pub thread_on: u128,
    pub attack_url: String,
}

pub static PUB_VAR: Lazy<Mutex<GlobalVar>> = Lazy::new(|| {
    Mutex::new(GlobalVar {
        thread_on: 0,
        attack_url: "String".to_owned(),
    })
});

pub static mut ATTACK_URL: Lazy<String> = Lazy::new(|| { String::from("") });


pub static mut AMOUNT: u128 = 0;

pub static mut THREADS_ON: u128 = 0;
