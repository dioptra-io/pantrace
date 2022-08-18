# pantrace

[![Tests](https://img.shields.io/github/workflow/status/dioptra-io/pantrace/Tests)](https://github.com/dioptra-io/pantrace/actions/workflows/tests.yml)
[![Coverage](https://img.shields.io/codecov/c/github/dioptra-io/pantrace)](https://app.codecov.io/gh/dioptra-io/pantrace)
[![crates.io](https://img.shields.io/crates/v/pantrace?logo=crates)](https://crates.io/crates/pantrace/)
[![docs.rs](https://img.shields.io/docsrs/pantrace)](https://docs.rs/pantrace/)

Convert between traceroute formats.

## Quickstart

```bash
docker run ghcr.io/dioptra-io/pantrace --help
```

```bash
curl -L -o example.ndjson \
    "https://atlas.ripe.net/api/v2/measurements/23119200/results/?start=1625097600&stop=1625788799&format=txt&probe_ids=6479"
cat example.ndjson | docker run ghcr.io/dioptra-io/pantrace --from atlas --to warts > example.warts
```
