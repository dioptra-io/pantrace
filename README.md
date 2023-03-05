# 〰️ pantrace

[![Coverage](https://img.shields.io/codecov/c/github/dioptra-io/pantrace?logo=codecov&logoColor=white)](https://app.codecov.io/gh/dioptra-io/pantrace)
[![crates.io](https://img.shields.io/crates/v/pantrace?logo=rust)](https://crates.io/crates/pantrace/)
[![docs.rs](https://img.shields.io/docsrs/pantrace?logo=docs.rs)](https://docs.rs/pantrace/)
[![Tests](https://img.shields.io/github/actions/workflow/status/dioptra-io/pantrace/tests.yml?logo=github&label=tests)](https://github.com/dioptra-io/pantrace/actions/workflows/tests.yml)

Pantrace converts between traceroute formats, in the same way as [Pandoc](https://pandoc.org) converts between document formats.

Each format needs to implement only two conversions: to and from the internal format.

## Quickstart

### Cargo

```bash
cargo install pantrace && pantrace --help
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
cat example.ndjson | pantrace --standalone --from atlas --to scamper-trace-warts > example.warts

# Convert from a file to a file
pantrace --standalone --from atlas --to scamper-trace-warts --input example.ndjson --output example.warts
```


## Formats

- `atlas`: RIPE Atlas JSONL (read/write)
- `flat`: JSONL with one document per reply (write-only)
- `internal`: Pantrace internal format (read/write)
- `iris`: Iris JSONL format (read/write)
- `scamper-trace-warts`: Scamper traceroute in warts format (read/write)

### Implementing a new format

To add a new `CustomFormat` to the pantrace CLI ([`main.rs`](src/main.rs)), two structures must be implemented:
- `CustomTracerouteReader` which implements the `Iterator<Item = Result<Traceroute>>` trait.
- `CustomTracerouteWriter` which implements the [`TracerouteWriter`](src/traits.rs) trait, and in particular the
  `fn write_traceroute(&mut self, traceroute: &Traceroute) -> Result<()>` function
where [`Traceroute`](src/formats/internal/models.rs) is pantrace's internal traceroute format.

The conversion between `CustomFormat` and `Traceroute` can be implemented in any way, but the current formats are
implemented as follows:
- `CustomFormat` to `Traceroute` conversion in a `to_internal` module:
  - `impl From<CustomFormat> for Traceroute { ... }`
- `CustomFormat` from `Traceroute` conversion in a `from_internal` module
  - `impl From<Traceroute> for CustomFormat { ... }`
  - or `impl From<Traceroute> for Vec<CustomFormat> { ... }` if `CustomFormat` is a single path traceroute format
