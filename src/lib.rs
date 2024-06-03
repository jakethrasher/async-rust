pub mod document;
pub mod sensible_client;

use document::Document;
use sensible_client::SensibleClient;

#[derive(Debug)]
pub struct Sensible {
    pub document: Document,
}

impl Sensible {
    pub fn new(api_key: String) -> Sensible {
        let client = SensibleClient::new(api_key);
        Sensible {
            document: Document {
                client: client.clone(),
            },
        }
    }
}
