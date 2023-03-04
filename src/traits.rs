//! Traits that must be implemented by traceroute formats.
use crate::internal::Traceroute;

pub trait TracerouteReader = Iterator<Item = anyhow::Result<Traceroute>>;

pub trait TracerouteWriter {
    fn write_traceroute(&mut self, traceroute: &Traceroute) -> anyhow::Result<()>;
    fn write_preamble(&mut self) -> anyhow::Result<()> {
        Ok(()) /* do nothing by default */
    }
    fn write_epilogue(&mut self) -> anyhow::Result<()> {
        Ok(()) /* do nothing by default */
    }
}
