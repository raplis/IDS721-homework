use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{Value, json};

async fn subtract_thirty_two(event: Value, _: Context) -> Result<Value, Error> {
    println!("Received event: {:?}", event);
    match event.as_f64() {
        Some(fahrenheit) => Ok(json!({ "result": fahrenheit * 5.0 / 9.0 })),
        None => {
            if let Some(temp) = event.get("result").and_then(|v| v.as_f64()) {
                Ok(json!({ "result": temp * 5.0 / 9.0 }))
            } else {
                Err(Error::from("Expected a floating point input or an object with a 'result' floating point field"))
            }
        }
    }
}

// async fn subtract_thirty_two(event: Value, _: Context) -> Result<Value, Error> {
//     println!("Received event: {:?}", event);
//     match event.as_f64() {
//         Some(fahrenheit) => Ok(json!({ "temperature": fahrenheit - 32.0 })),
//         None => {
//             if let Some(temp) = event.get("temperature").and_then(|v| v.as_f64()) {
//                 Ok(json!({ "temperature": temp - 32.0 }))
//             } else {
//                 Err(Error::from("Expected a floating point input or an object with a 'temperature' floating point field"))
//             }
//         }
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(subtract_thirty_two);
    lambda_runtime::run(func).await?;
    Ok(())
}
