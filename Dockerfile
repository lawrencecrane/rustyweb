#== Image to build application ==#
FROM rust:alpine as builder

WORKDIR /home/rustyweb

COPY lib ./lib/
COPY app ./app/

WORKDIR /home/rustyweb/app

RUN cargo build --release

CMD ["cargo", "build", "--release"]


#== Image to run application ==#
FROM alpine as rustyweb

RUN addgroup -S rustyweb && adduser -S rustyweb -G rustyweb 
RUN mkdir /opt/rustyweb && chown rustyweb:rustyweb /opt/rustyweb

USER rustyweb

WORKDIR /opt/rustyweb

ENV PATH "/opt/rustyweb:$PATH"

COPY --from=builder /home/rustyweb/app/target/release/rustywebapp .

CMD ["rustywebapp"]
