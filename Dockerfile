FROM rust:1.80 as builder
WORKDIR /usr/src/manipulation-detector
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/manipulation-detector/target/release/manipulation-detector /app/manipulation-detector
EXPOSE 3000
CMD ["/app/manipulation-detector"]
