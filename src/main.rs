use crate::requester::Requester;
use requester::HttpMethod;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
pub mod requester;

#[tokio::main]
async fn main() {
    // get_document_types().await;
    // generate_upload_url().await;
    // extract_document().await;
    get_document().await;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentTypes {
    name: String,
    id: String,
    created: String,
    schema: HashMap<String, String>,
}

async fn get_document_types() {
    let url = String::from("https://api.sensible.so/v0/document_types");
    let method: HttpMethod<()> = HttpMethod::Get;

    let request = Requester::<Vec<DocumentTypes>, ()>::new(url, method);

    let response = request.requester().await;

    match response {
        Ok(res) => println!("{:#?}", res),
        Err(e) => eprintln!("{:?}", e),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadResponse {
    id: String,
    created: String,
    status: String,
    r#type: String,
    upload_url: String,
}

async fn generate_upload_url() {
    let url = "https://api.sensible.so/v0/generate_upload_url/{ADD DOCUMENT TYPE HERE}".to_string();

    let mut body = HashMap::new();
    body.insert("content_type".to_string(), "application/pdf".to_string());

    let method: HttpMethod<HashMap<String, String>> = HttpMethod::Post(body);

    let request = Requester::<UploadResponse, HashMap<String, String>>::new(url, method);

    let response = request.requester().await;

    match response {
        Ok(res) => println!("{:#?}", res),
        Err(e) => eprintln!("{:?}", e),
    }
}

pub async fn extract_document() {
    let url = String::from("ADD SENSIBLE URL HERE");

    let buffer = read_file().unwrap();
    let method = HttpMethod::Put(buffer);

    let request = Requester::<HashMap<String, String>, Vec<u8>>::new(url, method);
    let response = request.requester().await;

    match response {
        Ok(res) => println!("{:#?}", res),
        Err(e) => eprintln!("{:#?}", e),
    }
}

fn read_file() -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open("/Users/jakethrasher/Downloads/1040_2021_sample.pdf")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    return Ok(buffer);
}

#[derive(Debug, Deserialize)]
struct ExtractionResponse {
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
async fn get_document() {
    let url = "https://api.sensible.so/v0/documents/{ADD DOCUMENT ID}".to_string();
    let method: HttpMethod<()> = HttpMethod::Get;

    let request = Requester::<Value, ()>::new(url, method);

    let response = request.requester().await;

    match response {
        Ok(res) => println!("{:#?}", res),
        Err(e) => eprintln!("{:#?}", e),
    }
}
