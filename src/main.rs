#![feature(stdin_forwarders)]

use clap::{AppSettings, ArgEnum, Parser};
use pantrace::atlas::models::AtlasTraceroute;
use pantrace::atlas::reader::AtlasReader;
use pantrace::internal::models::TracerouteReply;
use pantrace::internal::reader::InternalReader;
use pantrace::iris::models::IrisTraceroute;
use pantrace::iris::reader::IrisReader;
use pantrace::warts_trace::reader::WartsReader;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

#[derive(ArgEnum, Clone, Debug, PartialEq)]
enum Format {
    Atlas,
    Internal,
    Iris,
    Warts,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
struct Args {
    /// Input file (stdin if not specified).
    #[clap(short, long)]
    input: Option<String>,
    /// Output file (stdout if not specified).
    #[clap(short, long)]
    output: Option<String>,
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

    let input: Box<dyn BufRead> = match args.input {
        Some(input_file) => {
            let f = File::open(input_file).unwrap();
            Box::new(BufReader::new(f))
        }
        None => Box::new(stdin().lock()),
    };

    let mut output: Box<dyn Write> = match args.output {
        Some(output_file) => {
            let f = File::create(output_file).unwrap();
            Box::new(f)
        }
        None => Box::new(stdout().lock()),
    };

    let reader: Box<dyn Iterator<Item = Vec<TracerouteReply>>> = match args.from {
        Format::Atlas => Box::new(AtlasReader::new(input)),
        Format::Internal => Box::new(InternalReader::new(input)),
        Format::Iris => Box::new(IrisReader::new(input)),
        Format::Warts => Box::new(WartsReader::new(input)),
    };

    for replies in reader {
        // TODO: Create Writer structs?
        match args.to {
            Format::Atlas => {
                let traceroute = AtlasTraceroute::from_internal(&replies);
                output
                    .write_all(&serde_json::to_vec(&traceroute).unwrap())
                    .unwrap();
                output.write_all("\n".as_ref()).unwrap();
            }
            Format::Internal => {
                for reply in replies {
                    output
                        .write_all(&serde_json::to_vec(&reply).unwrap())
                        .unwrap();
                    output.write_all("\n".as_ref()).unwrap();
                }
            }
            Format::Iris => {
                let traceroute = IrisTraceroute::from_internal(&replies);
                output
                    .write_all(&serde_json::to_vec(&traceroute).unwrap())
                    .unwrap();
                output.write_all("\n".as_ref()).unwrap();
            }
            Format::Warts => {
                todo!()
            }
        }
    }
}
