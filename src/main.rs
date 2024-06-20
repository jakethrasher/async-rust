use sensible_api::Sensible;

#[tokio::main]
async fn main() {
    let sensible = Sensible::new(dotenv::var("TOKEN").unwrap());

    let extraction_id = "39aa0399-519e-4088-b91d-57f2d7393a82".to_string();

    let response = sensible.document.retrieve_extraction(extraction_id).await;

    match response {
        Ok(res) => println!("{:#?}", res),
        Err(e) => eprintln!("{:?}", e),
    }
}
