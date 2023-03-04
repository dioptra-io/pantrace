FROM rustlang/rust:nightly as builder
WORKDIR /usr/src/pantrace
COPY . .
# https://github.com/rust-lang/cargo/issues/10781
# A proper solution would be to cross-compile instead of doing emulated builds on GH.
RUN cargo install --config net.git-fetch-with-cli=true --path .

FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/pantrace /usr/local/bin/pantrace
ENTRYPOINT ["pantrace"]
