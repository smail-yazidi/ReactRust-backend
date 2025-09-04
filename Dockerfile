# مرحلة البناء
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# مرحلة التشغيل
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend
CMD ["./backend"]
