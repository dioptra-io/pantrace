#![feature(stdin_forwarders)]

use clap::{ArgEnum, Parser};
use pantrace::format::PantraceFormat;
use pantrace::{AtlasTraceroute, IrisTraceroute, TracerouteReply};
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
    /// Output start/end markers (e.g. Warts CycleStart/CycleStop).
    #[clap(short, long)]
    standalone: bool,
}

fn main() {
    let args = Args::parse();
    let lines = io::stdin().lines();
    let mut internal: Vec<TracerouteReply>;
    // TODO: Iterate on bytes instead? (for Ark)
    //   Or make from_bytes return the rest?
    for line in lines {
        println!("{}", line.as_ref().unwrap());
        match args.from {
            Format::Atlas => {
                internal = AtlasTraceroute::from_bytes(line.unwrap().as_bytes())
                    .unwrap()
                    .to_internal();
            }
            Format::Iris => {
                internal = IrisTraceroute::from_bytes(line.unwrap().as_bytes())
                    .unwrap()
                    .to_internal();
            }
            Format::Warts => {
                // TODO: Dereference addresses.
                internal = Traceroute::from_bytes(line.unwrap().as_bytes())
                    .unwrap()
                    .to_internal();
            }
        }
        match args.to {
            Format::Atlas => {
                io::stdout()
                    .write_all(
                        AtlasTraceroute::from_internal(&internal)
                            .to_bytes()
                            .as_slice(),
                    )
                    .unwrap();
            }
            Format::Iris => {
                io::stdout()
                    .write_all(
                        IrisTraceroute::from_internal(&internal)
                            .to_bytes()
                            .as_slice(),
                    )
                    .unwrap();
            }
            Format::Warts => {
                io::stdout()
                    .write_all(Traceroute::from_internal(&internal).to_bytes().as_slice())
                    .unwrap();
            }
        }
    }
}
