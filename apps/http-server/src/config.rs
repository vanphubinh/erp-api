use std::{env, net::SocketAddr};

#[derive(Debug, Clone)]
pub struct Config {
    pub addr: SocketAddr,
    pub db_url: String,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);

        let addr = format!("{host}:{port}")
            .parse()
            .unwrap_or_else(|_| "127.0.0.1:3000".parse().unwrap());

        let db_url = env::var("DATABASE_URL")?;

        Ok(Self { addr, db_url })
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:3000".parse().unwrap(),
            db_url: String::new(),
        }
    }
}
