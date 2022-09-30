use std::{thread, time};
use reqwest::Response;
use crate::ram_manger::UNSAFE_PUB_VAR;

pub(crate) fn time_function() {
    unsafe {
        UNSAFE_PUB_VAR.threads_on += 1;
        loop {
            UNSAFE_PUB_VAR.amount_sent = 0;
            thread::sleep(time::Duration::from_millis(10));
        }
    }
}

pub(crate) async fn request(url: &String) -> Result<Response, reqwest::Error> {
    reqwest::Client::new().get(url).send().await
}
