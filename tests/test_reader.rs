use anyhow::Result;
use pantrace::atlas::AtlasReader;
use pantrace::internal::models::TracerouteReply;
use pantrace::iris::IrisReader;
use pantrace::warts_trace::WartsTraceReader;
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_atlas() {
    let file = File::open("data/atlas.jsonl").unwrap();
    let reader = AtlasReader::new(BufReader::new(file));
    let results: Vec<Result<Vec<TracerouteReply>>> = reader.collect();
    for result in &results {
        assert!(result.is_ok());
    }
    assert_eq!(results.len(), 14);
}

#[test]
fn test_iris() {
    let file = File::open("data/iris.jsonl").unwrap();
    let reader = IrisReader::new(BufReader::new(file));
    let results: Vec<Result<Vec<TracerouteReply>>> = reader.collect();
    for result in &results {
        assert!(result.is_ok());
    }
    assert_eq!(results.len(), 1000);
}

#[test]
fn test_warts_trace() {
    let file = File::open("data/trace.warts").unwrap();
    let reader = WartsTraceReader::new(BufReader::new(file));
    let results: Vec<Result<Vec<TracerouteReply>>> = reader.collect();
    for result in &results {
        assert!(result.is_ok());
    }
    assert_eq!(results.len(), 1000);
}

// TODO: internal
