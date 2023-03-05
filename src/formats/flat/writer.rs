use crate::formats::flat::FlatTracerouteReply;
use std::io::Write;

use crate::formats::internal::Traceroute;
use crate::traits::TracerouteWriter;

pub struct FlatWriter<W: Write> {
    output: W,
}

impl<W: Write> FlatWriter<W> {
    pub fn new(output: W) -> FlatWriter<W> {
        FlatWriter { output }
    }
}

impl<W: Write> TracerouteWriter for FlatWriter<W> {
    fn write_traceroute(&mut self, traceroute: &Traceroute) -> anyhow::Result<()> {
        let replies: Vec<FlatTracerouteReply> = traceroute.into();
        let bytes = serde_json::to_vec(&replies)?;
        self.output.write_all(&bytes)?;
        self.output.write_all(b"\n")?;
        Ok(())
    }
}
