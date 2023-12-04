# docker build -t multi-stage -f multi-stage.dockerfile .
# docker run -it --rm --name multi-stage -p 8080:8080 multi-stage
FROM rust:1.65 AS builder

WORKDIR /build

RUN adduser --group --no-create-home --disabled-login --system builder
RUN chown -R builder /build
USER builder

RUN cargo new --bin devops
WORKDIR /build/devops

COPY Cargo.* ./
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/devops*

COPY ./src ./src
RUN cargo build --release

FROM scratch
COPY --from=builder /build/devops/target/release/devops /app
CMD ["/app"]