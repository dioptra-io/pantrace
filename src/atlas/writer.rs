use std::io::Write;

use crate::atlas::AtlasTraceroute;
use crate::internal::Traceroute;
use crate::traits::TracerouteWriter;

pub struct AtlasWriter<W: Write> {
    output: W,
}

impl<W: Write> AtlasWriter<W> {
    pub fn new(output: W) -> AtlasWriter<W> {
        AtlasWriter { output }
    }
}

impl<W: Write> TracerouteWriter for AtlasWriter<W> {
    fn write_traceroute(&mut self, traceroute: &Traceroute) -> anyhow::Result<()> {
        let traceroutes = AtlasTraceroute::from_internal(traceroute);
        for traceroute in traceroutes {
            let bytes = serde_json::to_vec(&traceroute)?;
            self.output.write_all(&bytes)?;
            self.output.write_all(b"\n")?;
        }
        Ok(())
    }
}
