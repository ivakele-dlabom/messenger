use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub cred_email: String,
    pub cred_app_password: String,
    pub sender_email: String,
    pub reciever_email: String

}


impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            cred_email: env::var("CRED_EMAIL")?,
            cred_app_password: env::var("CRED_PASSWORD")?,
            sender_email: env::var("SENDER_EMAIL")?, 
            reciever_email: env::var("RECIEVER_EMAIL")?,
        })
    }
}
