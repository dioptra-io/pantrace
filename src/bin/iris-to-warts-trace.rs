#![feature(stdin_forwarders)]

extern crate core;

use chrono::Utc;
use deku::DekuContainerWrite;
use iris_converters::IrisTraceroute;
use std::ffi::CString;
use std::io;
use std::io::Write;
use warts::{CycleStart, CycleStop, List, Object};

fn main() {
    let list_name = CString::new("default").unwrap();
    let hostname = CString::new("ubuntu-linux-20-04-desktop").unwrap();

    let mut list = List {
        length: 0,
        list_id: 1,
        list_id_human: 0,
        name: list_name.clone(),
        flags: Default::default(),
        param_length: None,
        description: Some(list_name.clone()),
        monitor_name: None,
    };
    list.fixup();
    io::stdout().write_all(Object::List(list).to_bytes().unwrap().as_slice());

    let mut cycle_start = CycleStart {
        length: 0,
        cycle_id: 1,
        list_id: 1,
        cycle_id_human: 0,
        start_time: Utc::now().timestamp() as u32,
        flags: Default::default(),
        param_length: None,
        stop_time: None,
        hostname: Some(hostname),
    };
    cycle_start.fixup();
    io::stdout().write_all(
        Object::CycleStart(cycle_start)
            .to_bytes()
            .unwrap()
            .as_slice(),
    );

    let lines = io::stdin().lines();
    for line in lines {
        let iris_t: IrisTraceroute = serde_json::from_str(line.as_ref().unwrap()).unwrap();
        let warts_t = iris_t.to_warts_trace();
        let data: Vec<u8> = Object::Traceroute(warts_t).to_bytes().unwrap();
        io::stdout().write_all(data.as_slice()).unwrap();
    }

    let mut cycle_stop = CycleStop {
        length: 0,
        cycle_id: 1,
        stop_time: Utc::now().timestamp() as u32,
        flags: Default::default(),
    };
    cycle_stop.fixup();
    io::stdout().write_all(Object::CycleStop(cycle_stop).to_bytes().unwrap().as_slice());
}
