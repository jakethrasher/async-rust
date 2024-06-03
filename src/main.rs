use sensible_api::Sensible;

#[tokio::main]
async fn main() {
    let sensible = Sensible::new(dotenv::var("TOKEN").unwrap());

    // let file_path = "/Users/jakethrasher/Downloads/1_extract_your_first_data.pdf".to_string();
    // let params = sensible_api::document::ExtractionParams {
    //     file_path,
    //     document_type: "senseml_basics".to_string(),
    // };

    // let response = sensible.document.async_extraction(params).await;

    let extraction_id = "39aa0399-519e-4088-b91d-57f2d7393a82".to_string();

    let response = sensible.document.retrieve_extraction(extraction_id).await;

    match response {
        Ok(res) => println!("{:#?}", res),
        Err(e) => eprintln!("{:?}", e),
    }
}
