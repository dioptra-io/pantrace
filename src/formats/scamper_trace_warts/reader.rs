use std::io::Read;

use warts::{Address, Object};

use crate::formats::internal::Traceroute;
use crate::formats::scamper_trace_warts::models::ScamperTraceWarts;

pub struct ScamperTraceWartsReader {
    cycle_id: u32,
    monitor_name: String,
    traceroutes: Vec<ScamperTraceWarts>,
}

impl ScamperTraceWartsReader {
    pub fn new<R: Read>(mut input: R) -> ScamperTraceWartsReader {
        let mut reader = ScamperTraceWartsReader {
            cycle_id: 0,
            monitor_name: "unknown".to_string(),
            traceroutes: Vec::new(),
        };

        // We currently read warts file in a single batch:
        // https://github.com/sharksforarms/deku/issues/105
        // Once this is implemented, we could move this logic to next().
        let mut data: Vec<u8> = Vec::new();
        input.read_to_end(&mut data).unwrap();
        let objects = Object::all_from_bytes(&data);

        // We assume that a warts file contains one and only one cycle; find the first one.
        for object in &objects {
            match object {
                Object::CycleDefinition(cycle_start) | Object::CycleStart(cycle_start) => {
                    let hostname = cycle_start.hostname.as_ref().unwrap().clone();
                    reader.cycle_id = cycle_start.cycle_id;
                    reader.monitor_name = hostname.into_string().unwrap();
                    break;
                }
                _ => {}
            }
        }

        // Old warts files, such as the ones produced by Ark around 2007-2010, use a global address
        // table, instead of an address table per traceroute as is the case in newer files.
        // As such, all the objects must be read before de-referencing the addresses contained in
        // the traceroutes. We could get rid of this by removing support for these old files.
        let mut table = Vec::new();
        for object in &objects {
            if let Object::Address(address) = object {
                table.push(Address::from(*address))
            }
        }

        // Dereference addresses and store the update traceroute object.
        for mut object in objects {
            if table.is_empty() {
                // Newer format, the address table is local to the traceroute.
                object.dereference();
            } else {
                // Older format, use the global address table.
                object.dereference_with_table(&table);
            }
            if let Object::Traceroute(traceroute) = object {
                reader.traceroutes.push(ScamperTraceWarts {
                    cycle_id: reader.cycle_id,
                    monitor_name: reader.monitor_name.to_string(),
                    traceroute,
                });
            }
        }
        reader
    }
}

impl Iterator for ScamperTraceWartsReader {
    type Item = anyhow::Result<Traceroute>;
    fn next(&mut self) -> Option<Self::Item> {
        self.traceroutes
            .pop()
            .map(|traceroute| Ok((&traceroute).into()))
    }
}
