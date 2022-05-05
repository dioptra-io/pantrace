#![feature(stdin_forwarders)]

use clap::{ArgEnum, Parser};
use pantrace::{
    AtlasReader, AtlasTraceroute, IrisReader, IrisTraceroute, TracerouteReply, WartsReader,
};
use std::io;

#[derive(ArgEnum, Clone, Debug, PartialEq)]
enum Format {
    Atlas,
    Internal,
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

fn main() {
    let args = Args::parse();
    let reader: Box<dyn Iterator<Item = Vec<TracerouteReply>>>;
    match args.from {
        Format::Atlas => {
            reader = Box::new(AtlasReader::new(io::stdin().lock()));
        }
        Format::Internal => {
            todo!()
        }
        Format::Iris => {
            reader = Box::new(IrisReader::new(io::stdin().lock()));
        }
        Format::Warts => {
            reader = Box::new(WartsReader::new(io::stdin().lock()));
        }
    }
    for replies in reader {
        // TODO: Create Writer structs?
        match args.to {
            Format::Atlas => {
                let traceroute = AtlasTraceroute::from_internal(&replies);
                println!("{}", serde_json::to_string(&traceroute).unwrap());
            }
            Format::Internal => {
                for reply in replies {
                    println!("{}", serde_json::to_string(&reply).unwrap());
                }
            }
            Format::Iris => {
                let traceroute = IrisTraceroute::from_internal(&replies);
                println!("{}", serde_json::to_string(&traceroute).unwrap());
            }
            Format::Warts => {
                todo!()
            }
        }
    }
}
