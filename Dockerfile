FROM rust:1.78 AS builder

WORKDIR /app
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.78-slim AS runtime

WORKDIR /app
COPY --from=builder /app/target/release/newsletter_server newsletter_server
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./newsletter_server"]
