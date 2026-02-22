# DanPhoto API - imagen para Coolify / Docker
# Multi-stage: compilación en Rust, ejecución en imagen mínima

# --- Stage 1: build ---
FROM rust:1-bookworm AS builder

WORKDIR /app

# Copiar manifestos primero para aprovechar caché de dependencias
COPY Cargo.toml Cargo.lock ./

# Crear un binario ficticio para que cargo cachee las dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Código fuente real
COPY src ./src

# Recompilar solo el código de la app (las deps ya están cacheadas)
RUN touch src/main.rs && cargo build --release

# --- Stage 2: runtime ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/danphoto-api /app/danphoto-api

# La app escucha en 0.0.0.0:PORT (PORT por defecto 3000)
ENV PORT=3000
EXPOSE 3000

# Carpeta para uploads (theme-of-the-day); debe ser escribible
RUN mkdir -p /app/uploads/theme-of-the-day

CMD ["/app/danphoto-api"]
