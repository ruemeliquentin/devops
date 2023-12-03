FROM rust:latest as builder
WORKDIR /usr/src/app

RUN adduser --no-create-home --disabled-login --group --system www
RUN chown www -R /app
USER www

RUN cargo new --bin devops
WORKDIR /app/devops

COPY Cargo.* ./
RUN cargo build --release
RUN rm src/.rs
RUN rm -rf .git/ .gitignore

COPY ./src ./src
RUN rm ./target/release/deps/devops
RUN cargo build --release

CMD ./target/release/devops
