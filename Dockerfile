# 1. Aşama: Statik Derleme (Musl) Aşaması
FROM clux/muslrust:1.75.0-stable AS builder

WORKDIR /usr/src/manipulation-detector

# Bağımlılıkları önbelleğe almak için boş proje yapısı
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

# Asıl kaynak kodları kopyalayıp tamamen statik olarak derliyoruz
COPY src ./src
RUN touch src/main.rs
RUN cargo build --release

# 2. Aşama: Minimum Çalıştırma Aşaması (Alpine en hafif ve güvenlisidir)
FROM alpine:latest

# SSL sertifikalarını ekliyoruz (Ollama API istekleri için şart)
RUN apk --no-cache add ca-certificates

WORKDIR /app

# Derlenen statik binary dosyasını kopyalıyoruz
COPY --from=builder /usr/src/manipulation-detector/target/x86_64-unknown-linux-musl/release/manipulation-detector .

# Port tanımı
EXPOSE 3000

# Uygulamayı başlatıyoruz
CMD ["./manipulation-detector"]
