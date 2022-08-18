# pantrace

[![Build](https://img.shields.io/github/workflow/status/dioptra-io/pantrace/Build)](https://github.com/dioptra-io/pantrace/actions/workflows/build.yml)
[![Coverage](https://img.shields.io/codecov/c/github/dioptra-io/pantrace)](https://app.codecov.io/gh/dioptra-io/pantrace)
[![crates.io](https://img.shields.io/crates/v/pantrace?logo=crates)](https://crates.io/crates/pantrace/)
[![docs.rs](https://img.shields.io/docsrs/pantrace)](https://docs.rs/pantrace/)

Convert between traceroute formats.

## Installation

```bash
cargo install pantrace
```

## Usage

```bash
cat iris.jsonl | pantrace -f iris -t atlas > atlas.json
```
