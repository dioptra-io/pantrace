use std::fs::File;
use std::io::{BufRead, BufReader};

use pantrace::formats::atlas::AtlasTraceroute;
use pantrace::formats::internal::Traceroute;

#[test]
fn test_atlas() {
    let file = File::open("data/atlas.jsonl").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let original: AtlasTraceroute = serde_json::from_str(&line.unwrap()).unwrap();
        let internal: Traceroute = (&original).into();
        let recovered: Vec<AtlasTraceroute> = (&internal).into();
        assert_eq!(original, recovered[0]);
    }
}

#[test]
fn test_iris() {
    // TODO
}

#[test]
fn test_warts_trace() {
    // TODO
}
