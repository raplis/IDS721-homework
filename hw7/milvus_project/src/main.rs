use reqwest::Client;
use serde::Serialize;
use tokio;

#[derive(Serialize)]
struct QueryPayload {
    collectionName: String,
    outputFields: Vec<String>,
    filter: String,
    limit: i64,
    offset: i64,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = "http://localhost:19530/v1/vector/query";

    let payload = QueryPayload {
        collectionName: "medium_articles".to_string(),
        outputFields: vec!["id".to_string(), "title".to_string(), "link".to_string()],
        filter: "id > 100".to_string(),
        limit: 100,
        offset: 0,
    };

    let response = client
        .post(url)
        .json(&payload)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .send()
        .await?;

    if response.status().is_success() {
        let response_body = response.text().await?;
        println!("Response was successful. Body:\n{}", response_body);
    } else {
        eprintln!("Request failed with status: {}", response.status());
        // Optionally, print the error body if there is one
        if let Ok(error_body) = response.text().await {
            eprintln!("Error body:\n{}", error_body);
        }
    }

    Ok(())
}
