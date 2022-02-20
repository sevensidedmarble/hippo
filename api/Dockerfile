FROM rustlang/rust:nightly as builder

RUN cargo install diesel_cli --no-default-features --features postgres

COPY . .

RUN cargo build --release --bin hippo

FROM scratch

COPY --from=builder /app/target/release/api /usr/local/bin

CMD ["./usr/local/bin/hippo"]
