use std::io::Write;

use crate::formats::internal::Traceroute;
use crate::traits::TracerouteWriter;

pub struct InternalWriter<W: Write> {
    output: W,
}

impl<W: Write> InternalWriter<W> {
    pub fn new(output: W) -> InternalWriter<W> {
        InternalWriter { output }
    }
}

impl<W: Write> TracerouteWriter for InternalWriter<W> {
    fn write_traceroute(&mut self, traceroute: &Traceroute) -> anyhow::Result<()> {
        let bytes = serde_json::to_vec(traceroute)?;
        self.output.write_all(&bytes)?;
        self.output.write_all(b"\n")?;
        Ok(())
    }
}
