# iris-converters

[![Build](https://img.shields.io/github/workflow/status/dioptra-io/iris-converters/Build)](https://github.com/dioptra-io/iris-converters/actions/workflows/build.yml)
[![Coverage](https://img.shields.io/codecov/c/github/dioptra-io/iris-converters)](https://app.codecov.io/gh/dioptra-io/iris-converters)
[![crates.io](https://img.shields.io/crates/v/iris-converters?logo=crates)](https://crates.io/crates/iris-converters/)
[![docs.rs](https://img.shields.io/docsrs/iris-converters)](https://docs.rs/iris-converters/)

Convert Iris measurement data to RIPE Atlas, Scamper, etc. formats.

## Installation

```bash
cargo install iris-converters
```

## Usage

```bash
cat iris-data.jsonl | iris-to-atlas > atlas-data.jsonl
cat iris-data.jsonl | iris-to-warts-trace > scamper-data.warts
# ...
```
