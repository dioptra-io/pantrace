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
    type Item = anyhow::Result<Vec<TracerouteReply>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|result| {
            let line = result?;
            let replies = serde_json::from_str::<AtlasTraceroute>(&line)?;
            Ok(replies.to_internal())
        })
    }
}
