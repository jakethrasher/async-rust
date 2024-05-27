use reqwest::{header::CONTENT_TYPE, Body, Error};
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

#[derive(Debug)]
pub enum HttpMethod<P> {
    Get,
    Post(P),
    Put(P),
}

#[derive(Debug)]
pub struct Requester<T, P> {
    url: String,
    method: HttpMethod<P>,
    _marker: PhantomData<T>,
}

impl<T, P> Requester<T, P> {
    pub fn new(url: String, method: HttpMethod<P>) -> Self {
        Self {
            url,
            method,
            _marker: PhantomData,
        }
    }

    pub async fn requester(&self) -> Result<T, Error>
    where
        T: DeserializeOwned,
        P: Serialize,
    {
        let client = reqwest::Client::new();

        let request_builder = match &self.method {
            HttpMethod::Get => client.get(&self.url),
            HttpMethod::Post(data) => client.post(&self.url).json(data),
            HttpMethod::Put(data) => {
                let body = serde_json::to_vec(data).unwrap();
                client.put(&self.url).body(Body::from(body))
            }
        };

        let response = request_builder
            .bearer_auth(dotenv::var("TOKEN").unwrap())
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await?;
        println!("raw response {:#?}", response);

        let json: T = response.json().await?;

        return Ok(json);
    }
}
