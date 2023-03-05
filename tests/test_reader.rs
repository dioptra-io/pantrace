use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use pantrace::formats::atlas::AtlasReader;
use pantrace::formats::internal::Traceroute;
use pantrace::formats::scamper_trace_warts::ScamperTraceWartsReader;

#[test]
fn test_atlas() {
    let file = File::open("data/atlas.jsonl").unwrap();
    let reader = AtlasReader::new(BufReader::new(file));
    let results: Vec<Result<Traceroute>> = reader.collect();
    for result in &results {
        assert!(result.is_ok());
    }
    assert_eq!(results.len(), 14);
}

#[test]
fn test_iris() {
    // TODO
}

#[test]
fn test_warts_trace() {
    let file = File::open("data/trace.warts").unwrap();
    let reader = ScamperTraceWartsReader::new(BufReader::new(file));
    let results: Vec<Result<Traceroute>> = reader.collect();
    for result in &results {
        assert!(result.is_ok());
    }
    assert_eq!(results.len(), 1000);
}
