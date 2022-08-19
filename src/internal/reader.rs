use crate::internal::models::TracerouteReply;
use std::io::{BufRead, Lines};

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
    type Item = anyhow::Result<Vec<TracerouteReply>>;
    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Group by flow
        match self.lines.next() {
            Some(Ok(_line)) => {
                todo!()
                // serde_json::from_str::<TracerouteReply>(&line).unwrap_or(None)
            }
            Some(Err(_)) => None,
            None => None,
        }
    }
}
