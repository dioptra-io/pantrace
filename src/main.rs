#![feature(stdin_forwarders)]

use clap::{ArgEnum, Parser};
use iris_converters::convertable::PantraceFormat;
use iris_converters::{AtlasTraceroute, IrisTraceroute, TracerouteReply};
use std::io;
use std::io::Write;
use warts::Traceroute;

#[derive(ArgEnum, Clone, Debug)]
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
    /// Output start/end markers (e.g. Warts CycleStart/CycleStop)
    #[clap(short, long)]
    standalone: bool,
}

fn main() {
    let args = Args::parse();
    let lines = io::stdin().lines();
    let mut internal: Vec<TracerouteReply>;
    for line in lines {
        // TODO: Optimize
        match args.from {
            Format::Atlas => {
                let atlas_t: AtlasTraceroute =
                    serde_json::from_str(line.as_ref().unwrap()).unwrap();
                internal = atlas_t.to_internal();
            }
            Format::Iris => {
                let iris_t: IrisTraceroute = serde_json::from_str(line.as_ref().unwrap()).unwrap();
                internal = iris_t.to_internal();
            }
            Format::Warts => {
                todo!()
            }
        }
        match args.to {
            // TODO: Dispatch on PantraceFormat instead of match?
            Format::Atlas => {
                let t = AtlasTraceroute::from_internal(&internal).unwrap();
                io::stdout().write_all(t.to_bytes().as_slice()).unwrap();
            }
            Format::Iris => {
                let t = IrisTraceroute::from_internal(&internal).unwrap();
                io::stdout().write_all(t.to_bytes().as_slice()).unwrap();
            }
            Format::Warts => {
                let t = Traceroute::from_internal(&internal).unwrap();
                io::stdout().write_all(t.to_bytes().as_slice()).unwrap();
            }
        }
    }
}
