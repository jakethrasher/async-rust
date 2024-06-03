// use reqwest::{header::CONTENT_TYPE, Body, Error};
// use serde::{de::DeserializeOwned, Serialize};
// use std::marker::PhantomData;

// #[derive(Debug)]
// pub enum HttpMethod<P> {
//     Get,
//     Post(P),
//     Put(P),
// }

// #[derive(Debug)]
// pub struct Requester<T, P> {
//     url: String,
//     method: HttpMethod<P>,
//     _marker: PhantomData<T>,
// }

// impl<T, P> Requester<T, P> {
//     // pub fn new(url: String, method: HttpMethod<P>) -> Self {
//     //     Self {
//     //         url,
//     //         method,
//     //         _marker: PhantomData,
//     //     }
//     // }

//     pub async fn request<T, P>(method, url) -> Result<T, Error>
//     where
//         T: DeserializeOwned,
//         P: Serialize,
//     {
//         let client = reqwest::Client::new();

//         let request_builder = match method {
//             HttpMethod::Get => client.get(url),
//             HttpMethod::Post(data) => client.post(url).json(data),
//             HttpMethod::Put(data) => {
//                 let body = serde_json::to_vec(data).unwrap();
//                 client.put(&self.url).body(Body::from(body))
//             }
//         };

//         let response = request_builder
//             .bearer_auth(dotenv::var("TOKEN").unwrap())
//             .header(CONTENT_TYPE, "application/json")
//             .send()
//             .await?;
//         println!("raw response {:#?}", response);

//         let json: T = response.json().await?;

//         return Ok(json);
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct DocumentTypes {
//     name: String,
//     id: String,
//     created: String,
//     schema: HashMap<String, String>,
// }

// async fn get_document_types() {
//     let url = String::from("https://api.sensible.so/v0/document_types");
//     let method: HttpMethod<()> = HttpMethod::Get;

//     let request = Requester::<Vec<DocumentTypes>, ()>::new(url, method);

//     let response = request.requester().await;

//     match response {
//         Ok(res) => println!("{:#?}", res),
//         Err(e) => eprintln!("{:?}", e),
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UploadResponse {
//     id: String,
//     created: String,
//     status: String,
//     r#type: String,
//     upload_url: String,
// }

// async fn generate_upload_url() {
//     let url = "https://api.sensible.so/v0/generate_upload_url/{ADD DOCUMENT TYPE HERE}".to_string();

//     let mut body = HashMap::new();
//     body.insert("content_type".to_string(), "application/pdf".to_string());

//     let method: HttpMethod<HashMap<String, String>> = HttpMethod::Post(body);

//     let request = Requester::<UploadResponse, HashMap<String, String>>::new(url, method);

//     let response = request.requester().await;

//     match response {
//         Ok(res) => println!("{:#?}", res),
//         Err(e) => eprintln!("{:?}", e),
//     }
// }

// pub async fn extract_document() {
//     let url = String::from("ADD SENSIBLE URL HERE");

//     let buffer = read_file().unwrap();
//     let method = HttpMethod::Put(buffer);

//     let request = Requester::<HashMap<String, String>, Vec<u8>>::new(url, method);
//     let response = request.requester().await;

//     match response {
//         Ok(res) => println!("{:#?}", res),
//         Err(e) => eprintln!("{:#?}", e),
//     }
// }

// fn read_file() -> Result<Vec<u8>, std::io::Error> {
//     let mut file = File::open("/Users/jakethrasher/Downloads/1040_2021_sample.pdf")?;
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer)?;

//     return Ok(buffer);
// }

// #[derive(Debug, Deserialize)]
// struct ExtractionResponse {
//     id: String,
//     created: String,
//     r#type: String,
//     status: String,
//     configuration: String,
//     parsed_document: HashMap<String, Value>,
//     validations: Vec<HashMap<String, Value>>,
//     file_metadata: HashMap<String, Value>,
//     validation_summary: HashMap<String, Value>,
//     errors: Vec<HashMap<String, Value>>,
//     completed: String,
//     classification_summary: Vec<HashMap<String, Value>>,
//     page_count: i8,
//     environment: String,
//     coverage: f64,
//     download_url: String,
// }
// async fn get_document() {
//     let url = "https://api.sensible.so/v0/documents/{ADD DOCUMENT ID}".to_string();
//     let method: HttpMethod<()> = HttpMethod::Get;

//     let request = Requester::<Value, ()>::new(url, method);

//     let response = request.requester().await;

//     match response {
//         Ok(res) => println!("{:#?}", res),
//         Err(e) => eprintln!("{:#?}", e),
//     }
// }
