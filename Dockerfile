# 1. Aşama: Derleme (Build) Aşaması
FROM rust:1.75-slim AS builder

WORKDIR /usr/src/manipulation-detector

# Bağımlılıkların önbelleğe alınması için boş bir proje oluşturup derliyoruz
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f src/main.rs target/release/deps/manipulation_detector*

# Şimdi asıl kaynak kodlarımızı kopyalıyoruz
COPY src ./src

# Projeyi production (release) modunda derliyoruz
RUN cargo build --release

# 2. Aşama: Çalıştırma (Runtime) Aşaması
FROM debian:bookworm-slim

# SSL bağlantıları (reqwest/Ollama API çağrıları) için gerekli sertifikaları kuruyoruz
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Sadece derlenen binary dosyasını ilk aşamadan buraya kopyalıyoruz
COPY --from=builder /usr/src/manipulation-detector/target/release/manipulation-detector .

# Backend'in dışarıya açacağı port
EXPOSE 3000

# Uygulamayı başlatıyoruz
CMD ["./manipulation-detector"]
