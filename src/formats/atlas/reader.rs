use std::io::{BufRead, Lines};

use crate::formats::atlas::AtlasTraceroute;
use crate::formats::internal::Traceroute;

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
    type Item = anyhow::Result<Traceroute>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|result| {
            let line = result?;
            let traceroute = serde_json::from_str::<AtlasTraceroute>(&line)?;
            Ok((&traceroute).into())
        })
    }
}
