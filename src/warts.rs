use crate::format::PantraceFormat;
use crate::TracerouteReply;
use chrono::{Duration, TimeZone, Utc};
use deku::DekuContainerWrite;
use std::net::Ipv6Addr;
use std::ops::{Add, Sub};
use warts::{Address, Object, Timeval, TraceProbe, TraceStopReason, TraceType, Traceroute};

impl PantraceFormat for Traceroute {
    fn from_bytes(_data: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        todo!()
    }
    fn to_bytes(self) -> Vec<u8> {
        Object::Traceroute(self).to_bytes().unwrap()
    }
    fn from_internal(replies: &[TracerouteReply]) -> Self
    where
        Self: Sized,
    {
        let mut t = Traceroute {
            length: 0,
            flags: Default::default(),
            param_length: None,
            list_id: Some(1),
            cycle_id: Some(1), // TODO: Should we put specific values here?
            src_addr_id: None,
            dst_addr_id: None,
            start_time: None, // TODO
            stop_reason: Some(TraceStopReason::Completed),
            stop_data: Some(0),
            trace_flags: None,
            attempts: None,
            hop_limit: None,
            trace_type: Some(TraceType::ICMPEchoParis),
            probe_size: None,
            src_port: Some(replies[0].probe_src_port),
            dst_port: Some(replies[0].probe_dst_port),
            first_ttl: None, // TODO
            ip_tos: None,
            timeout_sec: None,
            allowed_loops: None,
            hops_probed: None,
            gap_limit: None,
            gap_limit_action: None,
            loop_action: None,
            probes_sent: None,
            interval_csec: None,
            confidence_level: None,
            src_addr: Some(Address::from(replies[0].probe_src_addr)),
            dst_addr: Some(Address::from(replies[0].probe_dst_addr)),
            user_id: None,
            ip_offset: None,
            router_addr: None,
            hop_count: replies.len() as u16,
            hops: replies
                .iter()
                .map(warts_trace_probe_from_internal)
                .collect(),
            eof: 0,
        };
        t.fixup();
        t
    }
    fn to_internal(&self) -> Vec<TracerouteReply> {
        self.hops
            .iter()
            .map(|tp| {
                warts_trace_probe_to_internal(
                    tp,
                    self.trace_type.as_ref(),
                    self.src_addr,
                    self.dst_addr,
                    self.src_port,
                    self.dst_port,
                )
            })
            .collect()
    }
}

fn warts_trace_probe_from_internal(reply: &TracerouteReply) -> TraceProbe {
    let rtt_usec = (reply.rtt * 1000.0) as u32;
    let tx = Timeval::from(
        reply
            .capture_timestamp
            .sub(Duration::microseconds(rtt_usec as i64))
            .naive_utc(),
    );
    let mut tp = TraceProbe {
        flags: Default::default(),
        param_length: None,
        addr_id: None,
        probe_ttl: Some(reply.probe_ttl),
        reply_ttl: Some(reply.reply_ttl),
        hop_flags: None, // TODO
        probe_id: None,
        rtt_usec: Some(rtt_usec),
        icmp_type: Some(11),
        icmp_code: Some(0),
        probe_size: None,
        reply_size: Some(reply.reply_size),
        reply_ip_id: None,
        reply_ip_tos: None,
        next_hop_mtu: None,
        quoted_length: None,
        quoted_ttl: None, // TODO: Add all these fields.
        reply_tcp_flags: None,
        quoted_tos: None,
        icmp_extensions_length: None,
        icmp_extensions: vec![], // TODO
        addr: Some(Address::from(reply.reply_src_addr)),
        tx: Some(tx),
    };
    tp.fixup();
    tp
}

fn warts_trace_probe_to_internal(
    tp: &TraceProbe,
    trace_type: Option<&TraceType>,
    src_addr: Option<Address>,
    dst_addr: Option<Address>,
    src_port: Option<u16>,
    dst_port: Option<u16>,
) -> TracerouteReply {
    let tx = tp.tx.as_ref().unwrap_or(&Timeval {
        seconds: 0,
        microseconds: 0,
    });
    let capture_timestamp = Utc
        .timestamp(tx.seconds as i64, tx.microseconds * 1000)
        .add(Duration::microseconds(tp.rtt_usec.unwrap_or(0) as i64));
    TracerouteReply {
        probe_protocol: trace_type.map_or(0, protocol_number),
        probe_src_addr: src_addr.map_or(Ipv6Addr::from(0), ipv6_from_address),
        probe_dst_addr: dst_addr.map_or(Ipv6Addr::from(0), ipv6_from_address),
        probe_src_port: src_port.unwrap_or(0),
        probe_dst_port: dst_port.unwrap_or(0),
        capture_timestamp,
        probe_ttl: tp.probe_ttl.unwrap_or(0),
        reply_ttl: tp.reply_ttl.unwrap_or(0),
        reply_size: tp.reply_size.unwrap_or(0),
        mpls_labels: vec![], // TODO
        reply_src_addr: tp.addr.map_or(Ipv6Addr::from(0), ipv6_from_address),
        rtt: (tp.rtt_usec.unwrap_or(0) as f64) / 1000.0,
    }
}

fn ipv6_from_address(addr: Address) -> Ipv6Addr {
    match addr {
        Address::IPv4(_, x) => x.to_ipv6_mapped(),
        Address::IPv6(_, x) => x,
        _ => panic!("Unsupported address type"),
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
