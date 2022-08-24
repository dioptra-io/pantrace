use crate::atlas::models::AtlasTraceroute;
use crate::internal::TracerouteReply;
use crate::traits::TracerouteWriter;
use std::io::Write;

pub struct AtlasWriter<W: Write> {
    output: W,
}

impl<W: Write> AtlasWriter<W> {
    pub fn new(output: W) -> AtlasWriter<W> {
        AtlasWriter { output }
    }
}

impl<W: Write> TracerouteWriter for AtlasWriter<W> {
    fn write_traceroute(&mut self, replies: &[TracerouteReply]) -> anyhow::Result<()> {
        if !replies.is_empty() {
            let traceroute = AtlasTraceroute::from_internal(replies);
            let bytes = serde_json::to_vec(&traceroute)?;
            self.output.write_all(&bytes)?;
            self.output.write_all(b"\n")?;
        }
        Ok(())
    }
}
