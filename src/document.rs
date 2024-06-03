use crate::sensible_client::{HttpMethod, SensibleClient};
use reqwest::Error;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Document {
    pub client: SensibleClient,
}

impl Document {
    pub async fn async_extraction(
        &self,
        params: ExtractionParams,
    ) -> Result<HashMap<String, String>, Error> {
        // Generate upload URL via POST request
        let path = format!("generate_upload_url/{}", params.document_type);

        let mut body = HashMap::new();
        body.insert("content_type".to_string(), "application/pdf".to_string());

        let method = HttpMethod::Post(body);

        let response = self
            .client
            .request::<UploadResponse, HashMap<String, String>>(method, path)
            .await?;

        println!("UPLOAD URL RESPONSE {:#?}", response);
        // Send document buffer via PUT request
        let buffer = read_file(params.file_path);
        let method = HttpMethod::Put(buffer.unwrap());
        let response = self
            .client
            .request::<HashMap<String, String>, Vec<u8>>(method, response.upload_url)
            .await?;

        Ok(response)
    }

    pub async fn retrieve_extraction(&self, id: String) -> Result<ExtractionResponse, Error> {
        let path = format!("documents/{}", id);
        let response = self
            .client
            .request::<ExtractionResponse, ()>(HttpMethod::Get, path)
            .await?;

        Ok(response)
    }
}

#[derive(Debug, Deserialize)]
pub struct UploadResponse {
    id: String,
    created: String,
    status: String,
    r#type: String,
    upload_url: String,
}
#[derive(Debug, Deserialize)]
pub struct ExtractionParams {
    pub document_type: String,
    pub file_path: String,
}

#[derive(Debug, Deserialize)]
pub struct ExtractionResponse {
    id: String,
    created: String,
    r#type: String,
    status: String,
    configuration: String,
    parsed_document: HashMap<String, Value>,
    validations: Vec<HashMap<String, Value>>,
    file_metadata: HashMap<String, Value>,
    validation_summary: HashMap<String, Value>,
    errors: Vec<HashMap<String, Value>>,
    completed: String,
    classification_summary: Vec<HashMap<String, Value>>,
    page_count: i8,
    environment: String,
    coverage: f64,
    download_url: String,
}

fn read_file(path: String) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    return Ok(buffer);
}
