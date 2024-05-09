#!/bin/sh
if [ -z "$AWS_LAMBDA_RUNTIME_API" ]; then
  exec /usr/local/bin/aws-lambda-rie /app/target/release/hugging_face_transformer
else
  exec /app/target/release/hugging_face_transformer
fi