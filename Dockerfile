# مرحلة البناء
FROM rust:1.81 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# مرحلة التشغيل
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend
CMD ["./backend"]
