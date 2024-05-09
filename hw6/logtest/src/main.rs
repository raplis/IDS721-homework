use lambda_runtime::{handler_fn, Context, Error};
use log::{info, error};
use serde_json::Value;
use env_logger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    lambda_runtime::run(handler_fn(func)).await?;

    Ok(())
}

async fn func(event: Value, _: Context) -> Result<Value, Error> {
    info!("Dealing with the stuff: {:?}", event);
    error!("This is an error message. It should be in the CloudWatch logs.");
    Ok(event)
}


