# Homework 10 - Rust Serverless Transformer Endpoint

This project showcases the deployment of a Rust-based text transformer, leveraging Hugging Face's powerful models, within an AWS Lambda function containerized through Docker. 

## Project Structure

- `src/main.rs`: The Rust application entry point, setting up the AWS Lambda function handler.

  ```rust
  //due to the https body you cannot simply get the content from the event 
      println!("Event: {}", event);
      let body_str = event["body"].as_str().unwrap_or("");
      let body: Value = from_str(body_str).map_err(|e| e.to_string())?;
      let input_text = body["input_text"].as_str().unwrap_or("");
  ```

  ```json
  Event: 
  {
      "body": "{\"input_text\": \"hi\"}",
      "headers": {
          "accept": "*/*",
          "accept-encoding": "gzip,deflate",
          "content-length": "20",
          "content-type": "application/json",
          "host": "zofuppfzdooy5gwadlpmmxruau0zbxln.cell-1-lambda-url.us-east-1.on.aws",
          "user-agent": "curl/7.81.0",
          "x-amzn-tls-cipher-suite": "TLS_AES_128_GCM_SHA256",
          "x-amzn-tls-version": "TLSv1.3",
          "x-amzn-trace-id": "Self=1-66143103-66f1e9977c418adb14e52404;Root=1-66143103-399d7e870022ec3835903fec",
          "x-forwarded-for": "76.36.238.121",
          "x-forwarded-port": "443",
          "x-forwarded-proto": "https"
  ```

  below is part of the event content/

- `Dockerfile`: Instructions for Docker to build the container image.

  ```dockerfile
  FROM python:3.9
  
  RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
  ENV PATH="/root/.cargo/bin:${PATH}"
  
  RUN pip install transformers torch
  
  WORKDIR /app
  
  COPY ./src /app/src
  COPY ./Cargo.toml /app/Cargo.toml
  COPY ./Cargo.lock /app/Cargo.lock
  COPY ./transformer_inference.py /app/transformer_inference.py
  
  COPY ./entrypoint.sh ./entrypoint.sh
  RUN chmod +x ./entrypoint.sh
  
  RUN cargo build --release
  
  ADD https://github.com/aws/aws-lambda-runtime-interface-emulator/releases/latest/download/aws-lambda-rie /usr/local/bin/aws-lambda-rie
  RUN chmod +x /usr/local/bin/aws-lambda-rie
  
  ENV PATH="/app/target/release:${PATH}"
  
  ENTRYPOINT ["/app/entrypoint.sh"]
  
  ```

  

- `Cargo.toml` & `Cargo.lock`: Rust project's dependency specification.

- `transformer_inference.py`: Python script for performing inference using Hugging Face models.

- `entrypoint.sh`: Shell script to facilitate the execution of the Rust binary or the AWS Lambda RIE (Runtime Interface Emulator) for local testing.

## Setup Instructions

### Dockerizing the Rust Transformer

1. **Build the Docker Image**: Compile the Rust application and package it with its dependencies and the Python runtime environment into a Docker container.

   ```sh
   #before that  you should log in to the aws ecr
   sudo docker build -t rust_python_transformer .
   ```

2. **Tag the Docker Image for AWS ECR**: Prepare the Docker image for deployment by tagging it with the AWS ECR repository URI.

   ```sh
   sudo docker tag rust_python_transformer:latest 092592854796.dkr.ecr.us-east-1.amazonaws.com/rust_python_transformer:latest
   ```

3. **Push the Docker Image to AWS ECR**: Upload the Docker image to AWS ECR to make it accessible to AWS Lambda.

   ```sh
   sudo docker push 092592854796.dkr.ecr.us-east-1.amazonaws.com/rust_python_transformer:latest
   #the docker contains the pytorch, so it's pretty big
   ```

### Deploying to AWS Lambda

1. **Create an AWS Lambda Function**: From the AWS Management Console , create a new Lambda function based on a container image.
2. **Configure the Function**: Use the URI of the Docker image uploaded to ECR as the source of the Lambda function's container image.
3. **Set Up the Function URL**: Enable HTTP request handling by configuring a function URL and make it public.

### Testing the Endpoint

With the deployment complete, test the endpoint using a `curl` request:

```sh

curl -X POST https://vm2nkqlsmsmumlcrnfjcgdkjg40xirdb.lambda-url.us-east-1.on.aws/  -H "Content-Type: application/json"      -d '{"input_text": "hi"}'

```

the result is 

```
{"output":"[{\"label\": \"POSITIVE\", \"score\": 0.9983267188072205}]\n"}
```

you can test it by changing the text "hi" by your own words.
