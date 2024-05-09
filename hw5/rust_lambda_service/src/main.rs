use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, GetItemInput, AttributeValue};
use std::collections::HashMap;
use serde_json::{to_string, Value};

#[derive(Deserialize)] 
struct CustomEvent {
    book_id: String,
}

#[derive(Deserialize)]
struct LambdaEvent {
    body: String,
}

#[derive(Serialize)]
struct CustomOutput {
    title: String,
    author: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: Value, _: Context) -> Result<CustomOutput, Error> {
    // let event_json = to_string(&event).unwrap_or_else(|_| "Failed to serialize event".to_string());
    // println!("Received event: {}", event_json);
    let lambda_event: LambdaEvent = serde_json::from_value(event)?;
    let custom_event: CustomEvent = serde_json::from_str(&lambda_event.body)?;
    let client = DynamoDbClient::new(Region::UsEast1);
    let mut key = HashMap::new();
    key.insert(
        "bookId".to_string(),
        AttributeValue {
            s: Some(custom_event.book_id.clone()),
            ..Default::default()
        }
    );

    let get_item_input = GetItemInput {
        table_name: "Books".to_string(),
        key,
        ..Default::default()
    };

    let result = client.get_item(get_item_input).await?;
    let item = result.item.ok_or_else(|| "Book not found".to_string())?;

    let title = item["title"].s.as_ref().ok_or_else(|| "Title not found".to_string())?.to_string();
    let author = item["author"].s.as_ref().ok_or_else(|| "Author not found".to_string())?.to_string();

    Ok(CustomOutput { title, author })
}

