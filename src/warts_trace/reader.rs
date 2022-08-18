use crate::internal::models::TracerouteReply;
use crate::warts_trace::to::warts_traceroute_to_internal;
use std::io::Read;
use warts::{Address, Object, Traceroute};

pub struct WartsReader {
    traceroutes: Vec<(u32, String, Traceroute)>,
}

impl WartsReader {
    pub fn new<R: Read>(mut input: R) -> WartsReader {
        let mut reader = WartsReader {
            traceroutes: Vec::new(),
        };
        // We currently read warts file in a single batch.
        // https://github.com/sharksforarms/deku/issues/105
        let mut data: Vec<u8> = Vec::new();
        input.read_to_end(&mut data).unwrap();
        let objects = Object::all_from_bytes(data.as_slice());
        let mut table = Vec::new();
        let mut cycle_id = 0;
        let mut monitor_name = "unknown".to_string();
        for object in &objects {
            match object {
                Object::Address(address) => table.push(Address::from(*address)),
                Object::CycleDefinition(cycle_start) | Object::CycleStart(cycle_start) => {
                    cycle_id = cycle_start.cycle_id_human;
                    monitor_name = cycle_start
                        .hostname
                        .as_ref()
                        .unwrap()
                        .clone()
                        .into_string()
                        .unwrap();
                }
                _ => {}
            }
        }
        for mut object in objects {
            if table.is_empty() {
                object.dereference();
            } else {
                object.dereference_with_table(&table);
            }
            if let Object::Traceroute(traceroute) = object {
                reader
                    .traceroutes
                    .push((cycle_id, monitor_name.clone(), traceroute))
            }
        }
        reader
    }
}

impl Iterator for WartsReader {
    type Item = Vec<TracerouteReply>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.traceroutes.pop() {
            Some((cycle_id, monitor_name, traceroute)) => Some(warts_traceroute_to_internal(
                &traceroute,
                cycle_id,
                &monitor_name,
            )),
            _ => None,
        }
    }
}
