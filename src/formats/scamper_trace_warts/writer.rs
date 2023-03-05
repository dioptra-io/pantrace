use std::ffi::CString;
use std::io::Write;

use chrono::Utc;
use warts::{CycleStart, CycleStop, DekuContainerWrite, List, Object};

use crate::formats::internal::Traceroute;
use crate::formats::scamper_trace_warts::models::ScamperTraceWarts;
use crate::traits::TracerouteWriter;

pub struct ScamperTraceWartsWriter<W: Write> {
    output: W,
}

impl<W: Write> ScamperTraceWartsWriter<W> {
    pub fn new(output: W) -> ScamperTraceWartsWriter<W> {
        ScamperTraceWartsWriter { output }
    }
}

impl<W: Write> TracerouteWriter for ScamperTraceWartsWriter<W> {
    fn write_traceroute(&mut self, traceroute: &Traceroute) -> anyhow::Result<()> {
        let traceroutes: Vec<ScamperTraceWarts> = traceroute.into();
        for traceroute in traceroutes {
            let bytes = Object::Traceroute(traceroute.traceroute).to_bytes()?;
            self.output.write_all(&bytes)?;
        }
        Ok(())
    }

    fn write_preamble(&mut self) -> anyhow::Result<()> {
        let list_name = CString::new("TODO").unwrap();
        let hostname = CString::new("TODO").unwrap();

        let list = List {
            length: 0,
            list_id: 1, // TODO
            list_id_human: 0,
            name: list_name.clone(),
            flags: Default::default(),
            param_length: None,
            description: Some(list_name),
            monitor_name: None,
        };
        let bytes = Object::List(list.finalize()).to_bytes()?;
        self.output.write_all(&bytes)?;

        let cycle_start = CycleStart {
            length: 0,
            cycle_id: 1, // TODO
            list_id: 1,  // TODO
            cycle_id_human: 0,
            start_time: Utc::now().timestamp() as u32, // TODO
            flags: Default::default(),
            param_length: None,
            stop_time: None,
            hostname: Some(hostname),
        };
        let bytes = Object::CycleStart(cycle_start.finalize()).to_bytes()?;
        self.output.write_all(&bytes)?;

        Ok(())
    }

    fn write_epilogue(&mut self) -> anyhow::Result<()> {
        let cycle_stop = CycleStop {
            length: 0,
            cycle_id: 1,                              // TODO
            stop_time: Utc::now().timestamp() as u32, // TODO
            flags: Default::default(),
        };
        let bytes = Object::CycleStop(cycle_stop.finalize()).to_bytes()?;
        self.output.write_all(&bytes)?;
        Ok(())
    }
}
