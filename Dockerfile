FROM rust:1.71.1 AS builder

RUN apt-get update && apt-get install -y protobuf-compiler pkg-config 

WORKDIR /usr/src/

RUN cargo new egpt-grpc

WORKDIR /usr/src/egpt-grpc

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release

COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc

COPY --from=builder /usr/local/cargo/bin/egpt-grpc .
COPY --from=builder /usr/src/egpt-grpc/sql .

EXPOSE 9000

EXPOSE 50051

CMD ["./egpt-grpc"]