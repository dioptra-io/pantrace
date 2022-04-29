#![feature(stdin_forwarders)]

use clap::Parser;
use iris_converters::convertable::Convertable;
use iris_converters::{AtlasTraceroute, IrisTraceroute};
use std::io;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    from: String,
    #[clap(short, long)]
    to: String,
}

fn main() {
    // let args = Args::parse();
    let lines = io::stdin().lines();
    for line in lines {
        let iris_t: IrisTraceroute = serde_json::from_str(line.as_ref().unwrap()).unwrap();
        let atlas_t = AtlasTraceroute::from_internal(&iris_t.to_internal());
        // let atlas_t = Traceroute::from_internal(&iris_t.to_internal());
        println!("{}", serde_json::to_string(&atlas_t).unwrap());
    }
}
