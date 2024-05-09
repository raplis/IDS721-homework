use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{Value, json, from_str};
use std::process::Command;
use std::str;

async fn run_command(event: Value, _: Context) -> Result<Value, Error> {
    println!("Event: {}", event);
    let body_str = event["body"].as_str().unwrap_or("");
    let body: Value = from_str(body_str).map_err(|e| e.to_string())?;
    let input_text = body["input_text"].as_str().unwrap_or("");

    println!("Input text: {}", input_text);
    let output = Command::new("python3")
        .arg("transformer_inference.py")
        .arg(input_text)
        .output()
        .expect("Failed to execute process");

    let output_str = if !output.stderr.is_empty() {
        format!("Error: {}", str::from_utf8(&output.stderr).unwrap())
    } else {
        str::from_utf8(&output.stdout).unwrap().to_string()
    };

    Ok(json!({ "output": output_str }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(run_command);
    lambda_runtime::run(func).await?;
    Ok(())
}
