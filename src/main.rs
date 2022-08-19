use anyhow::{Context, Result};
use clap::{AppSettings, ArgEnum, Parser};
use pantrace::atlas::{AtlasReader, AtlasWriter};
use pantrace::internal::{InternalReader, InternalWriter};
use pantrace::iris::{IrisReader, IrisWriter};
use pantrace::traits::{TracerouteReader, TracerouteWriter};
use pantrace::warts_trace::WartsReader;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

#[derive(ArgEnum, Clone, Debug, PartialEq)]
enum Format {
    Atlas,
    Internal,
    Iris,
    WartsTrace,
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

fn main() -> Result<()> {
    let args = Args::parse();

    let input: Box<dyn BufRead> = match args.input {
        Some(input_file) => {
            let f = File::open(&input_file)
                .with_context(|| format!("Failed to open input file {}", input_file))?;
            Box::new(BufReader::new(f))
        }
        None => Box::new(stdin().lock()),
    };

    let output: Box<dyn Write> = match args.output {
        Some(output_file) => {
            let f = File::create(&output_file)
                .with_context(|| format!("Failed to open output file {}", output_file))?;
            Box::new(f)
        }
        None => Box::new(stdout().lock()),
    };

    let reader: Box<TracerouteReader> = match args.from {
        Format::Atlas => Box::new(AtlasReader::new(input)),
        Format::Internal => Box::new(InternalReader::new(input)),
        Format::Iris => Box::new(IrisReader::new(input)),
        Format::WartsTrace => Box::new(WartsReader::new(input)),
    };

    let mut writer: Box<dyn TracerouteWriter> = match args.to {
        Format::Atlas => Box::new(AtlasWriter::new(output)),
        Format::Internal => Box::new(InternalWriter::new(output)),
        Format::Iris => Box::new(IrisWriter::new(output)),
        _ => unimplemented!(),
    };

    for result in reader {
        if let Err(e) = result.map(|replies| writer.write_traceroute(&replies)) {
            eprintln!("{}", e)
        }
    }

    Ok(())
}
