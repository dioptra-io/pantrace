use std::io::{BufRead, Lines};

use crate::internal::TracerouteReply;

pub struct InternalReader<R: BufRead> {
    _lines: Lines<R>,
}

impl<R: BufRead> InternalReader<R> {
    pub fn new(input: R) -> InternalReader<R> {
        InternalReader {
            _lines: input.lines(),
        }
    }
}

impl<R: BufRead> Iterator for InternalReader<R> {
    type Item = anyhow::Result<Vec<TracerouteReply>>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement by grouping replies with the same flow ID")
        // serde_json::from_str::<TracerouteReply>(&line)
    }
}
