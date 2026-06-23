use anyhow::Context;

pub struct Config {
    pub base_url: String,
    pub api_token: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            base_url: std::env::var("MACRONX_API_URL")
                .unwrap_or_else(|_| "http://localhost:5000".into()),
            api_token: std::env::var("MACRONX_API_TOKEN")
                .context("MACRONX_API_TOKEN must be set")?,
        })
    }
}
