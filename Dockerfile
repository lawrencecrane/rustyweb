#== Image to build client assets ==#
FROM node:lts-slim as client-builder

WORKDIR /home/client

COPY app/client/package.json .

RUN npm install

COPY app/client ./

RUN  npm run build

#== Image to build application ==#
FROM rust:alpine as builder

RUN apk add libc-dev

WORKDIR /home/rustyweb

#* Install dependencies to cache
RUN mkdir -p /home/rustyweb/lib/src \
    && touch /home/rustyweb/lib/src/lib.rs

COPY lib/Cargo* ./lib/

RUN cd /home/rustyweb/lib && cargo build --release
#*

#* Build the app
COPY lib ./lib/
COPY app ./app/

COPY --from=client-builder /home/client/dist ./app/client/dist/

WORKDIR /home/rustyweb/app

RUN cargo build --release
#*

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
