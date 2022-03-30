#![feature(stdin_forwarders)]

use std::io;
use iris_converters::IrisTraceroute;

fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        let iris_t: IrisTraceroute = serde_json::from_str(line.as_ref().unwrap()).unwrap();
        let atlas_t = iris_t.to_atlas_traceroute("TODO", "TODO");
        println!("{}", serde_json::to_string(&atlas_t).unwrap());
    }
}
