#== Image to build application ==#
FROM rust:alpine as builder

WORKDIR /home/rustyweb

COPY Cargo.toml .
COPY Cargo.lock .
COPY src ./src/

RUN cargo build --release

CMD ["cargo", "build", "--release"]


#== Image to run application ==#
FROM alpine as rustyweb

WORKDIR /opt/rustyweb

ENV PATH "/opt/rustyweb:$PATH"

COPY --from=builder /home/rustyweb/target/release/rustyweb .

CMD ["rustyweb"]
