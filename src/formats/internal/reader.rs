use std::io::{BufRead, Lines};

use crate::formats::internal::Traceroute;

pub struct InternalReader<R: BufRead> {
    lines: Lines<R>,
}

impl<R: BufRead> InternalReader<R> {
    pub fn new(input: R) -> InternalReader<R> {
        InternalReader {
            lines: input.lines(),
        }
    }
}

impl<R: BufRead> Iterator for InternalReader<R> {
    type Item = anyhow::Result<Traceroute>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lines.next().map(|result| {
            let line = result?;
            Ok(serde_json::from_str::<Traceroute>(&line)?)
        })
    }
}
