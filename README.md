# 〰️ pantrace

[![Coverage](https://img.shields.io/codecov/c/github/dioptra-io/pantrace?logo=codecov&logoColor=white)](https://app.codecov.io/gh/dioptra-io/pantrace)
[![crates.io](https://img.shields.io/crates/v/pantrace?logo=rust)](https://crates.io/crates/pantrace/)
[![docs.rs](https://img.shields.io/docsrs/pantrace?logo=docs.rs)](https://docs.rs/pantrace/)
[![Tests](https://img.shields.io/github/actions/workflow/status/dioptra-io/pantrace/tests.yml?logo=github&label=tests)](https://github.com/dioptra-io/pantrace/actions/workflows/tests.yml)

Pantrace converts between traceroute formats, in the same way as [Pandoc](https://pandoc.org) converts between document formats.

Each format needs to implement only two conversions: to and from the internal format.

## Installation

### Cargo

```bash
cargo install pantrace
```

### Docker

```bash
docker run ghcr.io/dioptra-io/pantrace:main --help
```

### Nix

```bash
nix run github:dioptra-io/pantrace -- --help
```

## Usage

```bash
# Fetch traceroute results from the RIPE Atlas API
curl -L -o example.ndjson \
    "https://atlas.ripe.net/api/v2/measurements/23119199/results/?start=1625097600&stop=1625788799&format=txt&probe_ids=6479"
  
# Convert from the standard input to the standard output
cat example.ndjson | pantrace --standalone --from atlas --to warts-trace > example.warts

# Convert from a file to a file
pantrace --standalone --from atlas --to warts-trace --input example.ndjson --output example.warts
```

TODO: Show example with bigquery and M-Lab traceroutes.
TODO: Compare/benchmarks with scamper sc_warts2json
