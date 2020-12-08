#[macro_use]
extern crate lazy_static;

use std::{fs::File, io::prelude::*};

use chrono::{DateTime, Local};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

const CPOLAR_LOG_FILE_PATH: &str = "cpolar.log";

#[derive(Deserialize, Serialize, Default)]
struct AppConfig {
    host: String,
    port: Option<String>,
    user: String,
    password: String,
    to: Vec<String>,
}

lazy_static! {
    static ref AC: AppConfig = {
        let mut config_file = File::open("setting.toml").unwrap();
        let mut config_string = String::new();
        config_file.read_to_string(&mut config_string).unwrap();

        let app_config: AppConfig = toml::from_str(config_string.as_str()).unwrap();
        app_config
    };
}

type Err = Result<(), Box<dyn std::error::Error>>;

fn send_email(subject: String, body: String) -> Err {
    let messages = Message::builder()
        .from(format!(" <{}>", AC.user).parse()?)
        .to(format!(" <{}>", AC.to[0]).parse()?)
        .subject(subject)
        .body(body)?;

    let credentials = Credentials::new(AC.user.to_string(), AC.password.to_string());

    let st = SmtpTransport::relay(AC.host.as_str())?
        .credentials(credentials)
        .build();

    match st.send(&messages) {
        Ok(_r) => println!(
            "email sent at {} successfully",
            DateTime::format(&Local::now(), "%Y-%m-%d %H:%M:%S")
        ),
        Err(e) => println!("email sending failed: {}", e),
    }
    Ok(())
}

fn main() -> Err {
    let subject;
    let mut body = String::new();

    match File::open(CPOLAR_LOG_FILE_PATH) {
        Ok(mut f) => {
            subject = "地址".to_string();
            f.read_to_string(&mut body)?;
        }
        Err(e) => {
            subject = "日志".to_string();
            body = format!("failed to open {} file : {}", CPOLAR_LOG_FILE_PATH, e);
        }
    }
    send_email(subject, body)?;
    Ok(())
}

mod tests {
    #[test]
    fn test_send_email() {
        super::send_email("666".to_string(), "9999999999999999999".to_string()).unwrap();
    }
}
