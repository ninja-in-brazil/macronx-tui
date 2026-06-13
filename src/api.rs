use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::{self, HeaderMap, HeaderValue};

use crate::models::{CreateInboxRequest, Inbox};

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: &str, api_token: &str) -> Self {
        let mut headers = HeaderMap::new();
        let auth_value = HeaderValue::from_str(&format!("Bearer {}", api_token))
            .expect("Invalid API token header value");
        headers.insert(header::AUTHORIZATION, auth_value);

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub fn list_inboxes(&self) -> Result<Vec<Inbox>> {
        let url = format!("{}/api/v1/inboxes", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .context("Failed to connect to API")?;

        if !resp.status().is_success() {
            anyhow::bail!("API returned {}", resp.status());
        }

        resp.json::<Vec<Inbox>>().context("Failed to parse inboxes response")
    }

    pub fn get_inbox(&self, id: u64) -> Result<Inbox> {
        let url = format!("{}/api/v1/inboxes/{}", self.base_url, id);
        let resp = self
            .client
            .get(&url)
            .send()
            .context("Failed to connect to API")?;

        if !resp.status().is_success() {
            anyhow::bail!("API returned {}", resp.status());
        }

        resp.json::<Inbox>().context("Failed to parse inbox response")
    }

    pub fn create_inbox(&self, req: CreateInboxRequest) -> Result<Inbox> {
        let url = format!("{}/api/v1/inboxes", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .context("Failed to connect to API")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().unwrap_or_default();
            anyhow::bail!("API returned {}: {}", status, body);
        }

        resp.json::<Inbox>().context("Failed to parse created inbox response")
    }
}
