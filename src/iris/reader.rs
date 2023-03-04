use std::io::{BufRead, Lines};

use crate::internal::TracerouteReply;
use crate::iris::IrisTraceroute;

pub struct IrisReader<R: BufRead> {
    lines: Lines<R>,
}

impl<R: BufRead> IrisReader<R> {
    pub fn new(input: R) -> IrisReader<R> {
        IrisReader {
            lines: input.lines(),
        }
    }
}

impl<R: BufRead> Iterator for IrisReader<R> {
    type Item = anyhow::Result<Vec<TracerouteReply>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|result| {
            let line = result?;
            let replies = serde_json::from_str::<IrisTraceroute>(&line)?;
            Ok(replies.to_internal())
        })
    }
}
