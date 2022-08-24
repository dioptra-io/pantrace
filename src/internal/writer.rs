use crate::internal::TracerouteReply;
use crate::traits::TracerouteWriter;
use std::io::Write;

pub struct InternalWriter<W: Write> {
    output: W,
}

impl<W: Write> InternalWriter<W> {
    pub fn new(output: W) -> InternalWriter<W> {
        InternalWriter { output }
    }
}

impl<W: Write> TracerouteWriter for InternalWriter<W> {
    fn write_traceroute(&mut self, replies: &[TracerouteReply]) -> anyhow::Result<()> {
        for reply in replies {
            let bytes = serde_json::to_vec(&reply)?;
            self.output.write_all(&bytes)?;
            self.output.write_all(b"\n")?;
        }
        Ok(())
    }
}
