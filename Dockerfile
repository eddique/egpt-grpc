FROM rust:1.71.1 AS builder

RUN apt-get update && apt-get install -y protobuf-compiler pkg-config 

WORKDIR /usr/src/

RUN cargo new egpt-gprc

WORKDIR /usr/src/egpt-gprc

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc

COPY --from=builder /usr/local/cargo/bin/egpt-gprc .

EXPOSE 9000

EXPOSE 50051

CMD ["./egpt-gprc"]