use std::fs::File;
use std::io::{BufRead, BufReader};

use pantrace::atlas::AtlasTraceroute;
use pantrace::internal::Traceroute;

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
    todo!()
}

#[test]
fn test_warts_trace() {
    todo!()
}
