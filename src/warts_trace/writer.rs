use crate::internal::models::TracerouteReply;
use crate::traits::TracerouteWriter;
use crate::warts_trace::from::warts_traceroute_from_internal;
use chrono::Utc;
use std::ffi::CString;
use std::io::Write;
use warts::{CycleStart, CycleStop, DekuContainerWrite, List, Object};

pub struct WartsTraceWriter<W: Write> {
    output: W,
}

impl<W: Write> WartsTraceWriter<W> {
    pub fn new(output: W) -> WartsTraceWriter<W> {
        WartsTraceWriter { output }
    }
}

impl<W: Write> TracerouteWriter for WartsTraceWriter<W> {
    fn write_traceroute(&mut self, replies: &[TracerouteReply]) -> anyhow::Result<()> {
        let traceroute = warts_traceroute_from_internal(replies);
        let bytes = Object::Traceroute(traceroute).to_bytes()?;
        self.output.write_all(&bytes)?;
        Ok(())
    }

    fn write_preamble(&mut self) -> anyhow::Result<()> {
        let list_name = CString::new("TODO").unwrap();
        let hostname = CString::new("TODO").unwrap();

        let mut list = List {
            length: 0,
            list_id: 1,
            list_id_human: 0,
            name: list_name.clone(),
            flags: Default::default(),
            param_length: None,
            description: Some(list_name),
            monitor_name: None,
        };
        list.fixup();
        let bytes = Object::List(list).to_bytes()?;
        self.output.write_all(&bytes)?;

        let mut cycle_start = CycleStart {
            length: 0,
            cycle_id: 1,
            list_id: 1,
            cycle_id_human: 0,
            // TODO
            start_time: Utc::now().timestamp() as u32,
            flags: Default::default(),
            param_length: None,
            stop_time: None,
            hostname: Some(hostname),
        };
        cycle_start.fixup();
        let bytes = Object::CycleStart(cycle_start).to_bytes()?;
        self.output.write_all(&bytes)?;

        Ok(())
    }

    fn write_epilogue(&mut self) -> anyhow::Result<()> {
        let mut cycle_stop = CycleStop {
            length: 0,
            cycle_id: 1,
            // TODO
            stop_time: Utc::now().timestamp() as u32,
            flags: Default::default(),
        };
        cycle_stop.fixup();
        let bytes = Object::CycleStop(cycle_stop).to_bytes()?;
        self.output.write_all(&bytes)?;
        Ok(())
    }
}
