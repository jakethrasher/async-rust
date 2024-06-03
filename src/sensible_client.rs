use reqwest::{header::CONTENT_TYPE, Body, Error};
use serde::{de::DeserializeOwned, Serialize};

static BASE_URL: &str = "https://api.sensible.so/v0/";

#[derive(Debug)]
pub enum HttpMethod<P> {
    Get,
    Post(P),
    Put(P),
}

#[derive(Debug, Default, Clone)]
pub struct SensibleClient {
    pub api_key: String,
    pub base_url: String,
}

impl SensibleClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: BASE_URL.to_string(),
        }
    }
    pub async fn request<T, P>(&self, method: HttpMethod<P>, path: String) -> Result<T, Error>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let client = reqwest::Client::new();
        let url = format!("{}{}", &self.base_url, path);
        let request_builder = match method {
            HttpMethod::Get => client
                .get(url)
                .header(CONTENT_TYPE, "application/json")
                .bearer_auth(&self.api_key),

            HttpMethod::Post(data) => client
                .post(&url)
                .json(&data)
                .header(CONTENT_TYPE, "application/json")
                .bearer_auth(&self.api_key),

            HttpMethod::Put(data) => {
                let body = serde_json::to_vec(&data).unwrap();
                client.put(&path).body(Body::from(body))
            }
        };

        let response: T = request_builder.send().await?.json().await?;

        return Ok(response);
    }
}
