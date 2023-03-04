use std::net::Ipv6Addr;
use std::ops::Add;

use chrono::{Duration, TimeZone, Utc};
use warts::{Address, Timeval, TraceProbe, TraceType, Traceroute as WartsTraceroute};

use crate::internal::{Traceroute, TracerouteFlow, TracerouteReply};

/// Build an array of [TracerouteReply] from a [Traceroute].
pub fn warts_trace_to_internal(
    traceroute: &WartsTraceroute,
    cycle_id: u32,
    monitor_name: &str,
) -> Traceroute {
    Traceroute {
        measurement_id: cycle_id.to_string(),
        agent_id: monitor_name.to_string(),
        start_time: Default::default(), // TODO
        // start_time: Utc
        //     .timestamp_opt(traceroute.start_time.map_or(|t| t.seconds, 0) as i64, 0)
        //     .unwrap(),
        end_time: Default::default(), // TODO
        probe_protocol: traceroute.trace_type.as_ref().map_or(0, protocol_number),
        probe_src_addr: traceroute
            .src_addr
            .map_or(Ipv6Addr::UNSPECIFIED, ipv6_from_address),
        probe_dst_addr: traceroute
            .dst_addr
            .map_or(Ipv6Addr::UNSPECIFIED, ipv6_from_address),
        flows: vec![TracerouteFlow {
            probe_src_port: traceroute.src_port.unwrap_or(0),
            probe_dst_port: traceroute.dst_port.unwrap_or(0),
            replies: traceroute
                .hops
                .iter()
                .map(warts_trace_probe_to_internal)
                .collect(),
        }],
    }
}

fn warts_trace_probe_to_internal(tp: &TraceProbe) -> TracerouteReply {
    let tx = tp.tx.as_ref().unwrap_or(&Timeval {
        seconds: 0,
        microseconds: 0,
    });
    let capture_timestamp = Utc
        .timestamp_opt(tx.seconds as i64, tx.microseconds * 1000)
        .unwrap()
        .add(Duration::microseconds(tp.rtt_usec.unwrap_or(0) as i64));
    TracerouteReply {
        capture_timestamp,
        probe_ttl: tp.probe_ttl.unwrap_or(0),
        quoted_ttl: tp.quoted_ttl.unwrap_or(0),
        reply_ttl: tp.reply_ttl.unwrap_or(0),
        reply_size: tp.reply_size.unwrap_or(0),
        reply_mpls_labels: vec![], // TODO
        reply_src_addr: tp.addr.map_or(Ipv6Addr::from(0), ipv6_from_address),
        reply_icmp_type: tp.icmp_type.unwrap_or(0),
        reply_icmp_code: tp.icmp_code.unwrap_or(0),
        rtt: (tp.rtt_usec.unwrap_or(0) / 100) as u16,
    }
}

fn ipv6_from_address(addr: Address) -> Ipv6Addr {
    match addr {
        Address::IPv4(_, x) => x.to_ipv6_mapped(),
        Address::IPv6(_, x) => x,
        _ => panic!("Unsupported address type: {addr:?}"),
    }
}

fn protocol_number(trace_type: &TraceType) -> u8 {
    // TODO: IPv6
    match trace_type {
        TraceType::ICMPEcho => 1,
        TraceType::UDP => 17,
        TraceType::TCP => 6,
        TraceType::ICMPEchoParis => 1,
        TraceType::UDPParis => 17,
        TraceType::TCPAck => 6,
    }
}
