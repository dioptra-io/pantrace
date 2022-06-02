use crate::internal::models::TracerouteReply;
use chrono::TimeZone;
use chrono::{Duration, Utc};
use std::net::Ipv6Addr;
use std::ops::Add;
use warts::{Address, Timeval, TraceProbe, TraceType, Traceroute};

pub fn warts_traceroute_to_internal(
    traceroute: &Traceroute,
    cycle_id: u32,
    monitor_name: &str,
) -> Vec<TracerouteReply> {
    traceroute
        .hops
        .iter()
        .map(|tp| {
            warts_trace_probe_to_internal(
                tp,
                cycle_id,
                monitor_name,
                traceroute.start_time.as_ref(),
                traceroute.trace_type.as_ref(),
                traceroute.src_addr,
                traceroute.dst_addr,
                traceroute.src_port,
                traceroute.dst_port,
            )
        })
        .collect()
}

fn warts_trace_probe_to_internal(
    tp: &TraceProbe,
    cycle_id: u32,
    monitor_name: &str,
    trace_start: Option<&Timeval>,
    trace_type: Option<&TraceType>,
    src_addr: Option<Address>,
    dst_addr: Option<Address>,
    src_port: Option<u16>,
    dst_port: Option<u16>,
) -> TracerouteReply {
    let traceroute_start = trace_start.unwrap_or(&Timeval {
        seconds: 0,
        microseconds: 0,
    });
    let tx = tp.tx.as_ref().unwrap_or(&Timeval {
        seconds: 0,
        microseconds: 0,
    });
    let capture_timestamp = Utc
        .timestamp(tx.seconds as i64, tx.microseconds * 1000)
        .add(Duration::microseconds(tp.rtt_usec.unwrap_or(0) as i64));
    TracerouteReply {
        measurement_id: cycle_id.to_string(),
        agent_id: monitor_name.into(),
        traceroute_start: Utc.timestamp(traceroute_start.seconds as i64, 0),
        probe_protocol: trace_type.map_or(0, protocol_number),
        probe_src_addr: src_addr.map_or(Ipv6Addr::from(0), ipv6_from_address),
        probe_dst_addr: dst_addr.map_or(Ipv6Addr::from(0), ipv6_from_address),
        probe_src_port: src_port.unwrap_or(0),
        probe_dst_port: dst_port.unwrap_or(0),
        capture_timestamp,
        probe_ttl: tp.probe_ttl.unwrap_or(0),
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
        _ => panic!("Unsupported address type: {:?}", addr),
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
