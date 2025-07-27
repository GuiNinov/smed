use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub aws_access_key: String,
    pub aws_secret_key: String,
}


impl Config {
    pub fn from_env(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(path);
        dotenvy::from_path(path).ok();

        Ok(Self {
            aws_access_key: env::var("AWS_ACCESS_KEY_ID")?,
            aws_secret_key: env::var("AWS_SECRET_ACCESS_KEY")?,
        })
    }
}
