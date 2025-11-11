use std::env;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: SocketAddr,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .unwrap_or(3000);

        let addr = format!("{}:{}", host, port)
            .parse::<SocketAddr>()
            .unwrap_or_else(|_| "127.0.0.1:3000".parse().unwrap());

        Ok(Config { addr })
    }
}
