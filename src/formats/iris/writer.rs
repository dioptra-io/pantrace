use std::io::Write;

use crate::formats::internal::Traceroute;
use crate::formats::iris::IrisTraceroute;
use crate::traits::TracerouteWriter;

pub struct IrisWriter<W: Write> {
    output: W,
}

impl<W: Write> IrisWriter<W> {
    pub fn new(output: W) -> IrisWriter<W> {
        IrisWriter { output }
    }
}

impl<W: Write> TracerouteWriter for IrisWriter<W> {
    fn write_traceroute(&mut self, traceroute: &Traceroute) -> anyhow::Result<()> {
        let traceroute: IrisTraceroute = traceroute.into();
        let bytes = serde_json::to_vec(&traceroute)?;
        self.output.write_all(&bytes)?;
        self.output.write_all(b"\n")?;
        Ok(())
    }
}
