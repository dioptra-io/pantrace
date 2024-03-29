use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::process::exit;

use anyhow::{Context, Result};
use clap::Parser;
use pantrace::formats::atlas::{AtlasReader, AtlasWriter};
use pantrace::formats::flat::FlatWriter;
use pantrace::formats::internal::{InternalReader, InternalWriter, Traceroute};
use pantrace::formats::iris::{IrisReader, IrisWriter};
use pantrace::formats::scamper_trace_warts::{ScamperTraceWartsReader, ScamperTraceWartsWriter};
use pantrace::traits::TracerouteWriter;

#[derive(Clone, Debug, PartialEq, clap::ValueEnum)]
enum Format {
    Atlas,
    Flat,
    Internal,
    Iris,
    ScamperTraceWarts,
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Input file (stdin if not specified).
    #[clap(short, long)]
    input: Option<String>,
    /// Output file (stdout if not specified).
    #[clap(short, long)]
    output: Option<String>,
    /// Input format.
    #[clap(short, long)]
    from: Format,
    /// Output format.
    #[clap(short, long)]
    to: Format,
    /// Output start/end markers (e.g. Warts CycleStart/CycleStop).
    #[clap(short, long)]
    standalone: bool,
    /// Exit on the first error, instead of logging them.
    #[clap(short, long)]
    exit_on_error: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input: Box<dyn BufRead> = match args.input {
        Some(input_file) => {
            let f = File::open(&input_file)
                .with_context(|| format!("Failed to open input file {input_file}"))?;
            Box::new(BufReader::new(f))
        }
        None => Box::new(stdin().lock()),
    };

    let output: Box<dyn Write> = match args.output {
        Some(output_file) => {
            let f = File::create(&output_file)
                .with_context(|| format!("Failed to open output file {output_file}"))?;
            Box::new(f)
        }
        None => Box::new(stdout().lock()),
    };

    let reader: Box<dyn Iterator<Item = Result<Traceroute>>> = match args.from {
        Format::Atlas => Box::new(AtlasReader::new(input)),
        Format::Flat => todo!("reading flat format is not supported"),
        Format::Internal => Box::new(InternalReader::new(input)),
        Format::Iris => Box::new(IrisReader::new(input)),
        Format::ScamperTraceWarts => Box::new(ScamperTraceWartsReader::new(input)),
    };

    let mut writer: Box<dyn TracerouteWriter> = match args.to {
        Format::Atlas => Box::new(AtlasWriter::new(output)),
        Format::Flat => Box::new(FlatWriter::new(output)),
        Format::Internal => Box::new(InternalWriter::new(output)),
        Format::Iris => Box::new(IrisWriter::new(output)),
        Format::ScamperTraceWarts => Box::new(ScamperTraceWartsWriter::new(output)),
    };

    if args.standalone {
        writer.write_preamble()?;
    }

    for result in reader {
        if let Err(e) = result.map(|replies| writer.write_traceroute(&replies)) {
            eprintln!("{e}");
            if args.exit_on_error {
                exit(1);
            }
        }
    }

    if args.standalone {
        writer.write_epilogue()?;
    }

    Ok(())
}
