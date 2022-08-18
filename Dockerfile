FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/pantrace
COPY . .
RUN cargo install --path .

FROM ubuntu:22.04
COPY --from=builder /usr/local/cargo/bin/pantrace /usr/local/bin/pantrace
ENTRYPOINT ["pantrace"]
