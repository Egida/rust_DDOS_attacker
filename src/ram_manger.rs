use std::sync::{Mutex};

use once_cell::sync::Lazy;

pub struct GlobalVar {
    pub thread_on: u128,
    pub attack_url: String,
    pub force: u64,
}

pub static PUB_VAR: Lazy<Mutex<GlobalVar>> = Lazy::new(|| {
    Mutex::new(GlobalVar {
        thread_on: 0,
        attack_url: "String".to_owned(),
        force: 0,
    })
});
