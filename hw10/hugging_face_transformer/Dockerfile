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


ADD https://github.com/aws/aws-lambda-runtime-interface-emulator/releases/latest/download/aws-lambda-rie /usr/local/bin/aws-lambda-rie
RUN chmod +x /usr/local/bin/aws-lambda-rie

ENV PATH="/app/target/release:${PATH}"

ENTRYPOINT ["/app/entrypoint.sh"]
