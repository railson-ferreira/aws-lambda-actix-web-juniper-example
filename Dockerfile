FROM rust:1

COPY ./ ./

RUN cargo lambda build --release --target aarch64-unknown-linux-gnu

CMD ["./target/lambda/bootstrap/bootstrap"]