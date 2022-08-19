use crate::internal::models::TracerouteReply;
use crate::iris::models::IrisTraceroute;
use crate::traits::TracerouteWriter;
use std::io::Write;

pub struct IrisWriter<W: Write> {
    output: W,
}

impl<W: Write> IrisWriter<W> {
    pub fn new(output: W) -> IrisWriter<W> {
        IrisWriter { output }
    }
}

impl<W: Write> TracerouteWriter for IrisWriter<W> {
    fn write_traceroute(&mut self, replies: &[TracerouteReply]) -> anyhow::Result<()> {
        let traceroute = IrisTraceroute::from_internal(replies);
        let bytes = serde_json::to_vec(&traceroute)?;
        self.output.write_all(&bytes)?;
        self.output.write_all(b"\n")?;
        Ok(())
    }
}
