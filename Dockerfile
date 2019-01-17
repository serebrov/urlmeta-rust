FROM rust:1.31

WORKDIR /usr/src/urlmeta
COPY ./app .

RUN cargo install --path .
