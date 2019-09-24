#== Image to build client assets ==#
FROM node:lts-slim as client-builder

WORKDIR /home/client

COPY app/client/package.json .

RUN npm install

COPY app/client ./

RUN  npm run build

#== Image to build application ==#
FROM rust:alpine as builder

WORKDIR /home/rustyweb

COPY lib ./lib/
COPY app ./app/
COPY --from=client-builder /home/client/dist ./app/client/dist/

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
