use crate::atlas::models::AtlasTraceroute;
use crate::internal::models::TracerouteReply;
use std::io::{BufRead, Lines};

pub struct AtlasReader<R: BufRead> {
    lines: Lines<R>,
}

impl<R: BufRead> AtlasReader<R> {
    pub fn new(input: R) -> AtlasReader<R> {
        AtlasReader {
            lines: input.lines(),
        }
    }
}

impl<R: BufRead> Iterator for AtlasReader<R> {
    type Item = Vec<TracerouteReply>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(line)) => serde_json::from_str::<AtlasTraceroute>(&line)
                .map(|t| Some(t.to_internal()))
                .unwrap_or(None),
            Some(Err(_)) => None,
            None => None,
        }
    }
}
