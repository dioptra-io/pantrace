use crate::internal::models::TracerouteReply;
use crate::iris::models::IrisTraceroute;
use std::io::{BufRead, Lines};

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
    type Item = Vec<TracerouteReply>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(line)) => serde_json::from_str::<IrisTraceroute>(&line)
                .map(|t| Some(t.to_internal()))
                .unwrap_or(None),
            Some(Err(_)) => None,
            None => None,
        }
    }
}
