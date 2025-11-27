# Multi-stage build for the web server

# Stage 1: Build the web server
FROM rust:latest as server-builder
WORKDIR /app
COPY . .
RUN cargo build --release -p web-server

# Stage 2: Build the web assets (WASM and static files)
FROM node:22 as web-builder
WORKDIR /app
COPY . .
RUN npm ci --workspaces --include-workspace-root
RUN npm run build

# Stage 3: Runtime image
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

# Copy the compiled web server
COPY --from=server-builder /app/target/release/web-server /app/web-server

# Copy the built web assets
COPY --from=web-builder /app/dist /app/dist

EXPOSE 3000
ENV RUST_LOG=info
CMD ["./web-server"]
