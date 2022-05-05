#![feature(stdin_forwarders)]

use clap::{ArgEnum, Parser};
use pantrace::format::PantraceFormat;
use pantrace::{warts_traceroute_to_internal, AtlasTraceroute, IrisTraceroute, TracerouteReply};
use std::io;
use std::io::{Read, Write};
use std::mem::transmute;
use warts::{Object, Traceroute};

#[derive(ArgEnum, Clone, Debug, PartialEq)]
enum Format {
    Atlas,
    Iris,
    Warts,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input format.
    #[clap(short, long, arg_enum)]
    from: Format,
    /// Output format.
    #[clap(short, long, arg_enum)]
    to: Format,
    /// Output start/end markers (e.g. Warts CycleStart/CycleStop).
    #[clap(short, long)]
    standalone: bool,
    // TODO: Option to ignore errors/print invalid lines.
}

trait TracerouteReader {
    fn done(&self) -> bool;
    fn next(&mut self) -> Vec<TracerouteReply>;
}

struct WartsReader {
    traceroutes: Vec<(u32, String, Traceroute)>,
}

impl WartsReader {
    pub fn new() -> WartsReader {
        let mut reader = WartsReader {
            traceroutes: vec![],
        };
        let mut data: Vec<u8> = Vec::new();
        io::stdin().read_to_end(&mut data).unwrap();
        let objects = Object::all_from_bytes(data.as_slice());
        let mut cycle_id = 0;
        let mut monitor_name = "unknown".to_string();
        for mut object in objects {
            object.dereference();
            match object {
                Object::CycleStart(cycle_start) => {
                    cycle_id = cycle_start.cycle_id_human;
                    monitor_name = cycle_start.hostname.unwrap().into_string().unwrap();
                }
                Object::Traceroute(traceroute) => {
                    reader
                        .traceroutes
                        .push((cycle_id, monitor_name.clone(), traceroute))
                }
                _ => {}
            }
        }
        reader
    }
}

impl TracerouteReader for WartsReader {
    fn done(&self) -> bool {
        self.traceroutes.is_empty()
    }
    fn next(&mut self) -> Vec<TracerouteReply> {
        if let (cycle_id, monitor_name, traceroute) = self.traceroutes.pop().unwrap() {
            return warts_traceroute_to_internal(&traceroute, cycle_id, &monitor_name);
        }
        vec![]
    }
}

fn main() {
    let args = Args::parse();
    // TODO: Cleanup this?
    if args.from == Format::Warts {
        let mut reader = WartsReader::new();
        while !reader.done() {
            let replies = reader.next();
            for reply in replies {
                println!("{}", serde_json::to_string(&reply).unwrap());
            }
        }
    } else {
        // let lines = io::stdin().lines();
        // let mut internal: Vec<TracerouteReply>;
        // // TODO: Iterate on bytes instead? (for Ark)
        // //   Or make from_bytes return the rest?
        // for line in lines {
        //     match args.from {
        //         Format::Atlas => {
        //             internal = AtlasTraceroute::from_bytes(line.unwrap().as_bytes())
        //                 .unwrap()
        //                 .to_internal();
        //         }
        //         Format::Iris => {
        //             internal = IrisTraceroute::from_bytes(line.unwrap().as_bytes())
        //                 .unwrap()
        //                 .to_internal();
        //         }
        //         Format::Warts => {
        //             // TODO: Dereference addresses.
        //             internal = Traceroute::from_bytes(line.unwrap().as_bytes())
        //                 .unwrap()
        //                 .to_internal();
        //         }
        //     }
        //     match args.to {
        //         Format::Atlas => {
        //             io::stdout()
        //                 .write_all(
        //                     AtlasTraceroute::from_internal(&internal)
        //                         .to_bytes()
        //                         .as_slice(),
        //                 )
        //                 .unwrap();
        //         }
        //         Format::Iris => {
        //             if let Some(t) = IrisTraceroute::from_internal_or_none(&internal) {
        //                 io::stdout().write_all(t.to_bytes().as_slice()).unwrap();
        //             }
        //         }
        //         Format::Warts => {
        //             io::stdout()
        //                 .write_all(Traceroute::from_internal(&internal).to_bytes().as_slice())
        //                 .unwrap();
        //         }
        //     }
        // }
    }
}
