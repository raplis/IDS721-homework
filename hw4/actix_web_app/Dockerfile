FROM rust:1.75.0 as builder

RUN USER=root cargo new --bin actix_web_app

WORKDIR /actix_web_app

COPY ./ ./

RUN cargo build --release

FROM ubuntu:22.04
ARG APP=/usr/src/app

RUN mkdir -p ${APP}

COPY --from=builder /actix_web_app/target/release/actix_web_app ${APP}/actix_web_app

WORKDIR ${APP}

CMD ["./actix_web_app"]