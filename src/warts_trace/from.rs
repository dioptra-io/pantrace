use std::ops::Sub;

use chrono::Duration;
use warts::{
    Address,
    Timeval,
    TraceProbe,
    TraceStopReason,
    TraceType,
    Traceroute as WartsTraceroute,
};

use crate::internal::{Traceroute, TracerouteReply};

/// Build a [Traceroute] from an array of [TracerouteReply].
/// There must be at-least one reply, and all replies must have the same flow identifier.
pub fn warts_trace_from_internal(traceroute: &Traceroute) -> Vec<WartsTraceroute> {
    traceroute
        .flows
        .iter()
        .map(|flow| {
            let mut t = WartsTraceroute {
                length: 0,
                flags: Default::default(),
                param_length: None,
                list_id: Some(1),
                cycle_id: Some(1), // TODO: Should we put specific values here?
                src_addr_id: None,
                dst_addr_id: None,
                start_time: Some(Timeval::from(traceroute.start_time.naive_utc())),
                stop_reason: Some(TraceStopReason::Completed),
                stop_data: Some(0),
                trace_flags: None,
                attempts: None,
                hop_limit: None,
                trace_type: Some(TraceType::ICMPEchoParis),
                probe_size: None,
                src_port: Some(flow.probe_src_port),
                dst_port: Some(flow.probe_dst_port),
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
                src_addr: Some(Address::from(traceroute.probe_src_addr)),
                dst_addr: Some(Address::from(traceroute.probe_dst_addr)),
                user_id: None,
                ip_offset: None,
                router_addr: None,
                hop_count: flow.replies.len() as u16,
                hops: flow
                    .replies
                    .iter()
                    .map(warts_trace_probe_from_internal)
                    .collect(),
                eof: 0,
            };
            t.fixup();
            t
        })
        .collect()
}

fn warts_trace_probe_from_internal(reply: &TracerouteReply) -> TraceProbe {
    let rtt_usec = (reply.rtt as u32) * 100;
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
        quoted_length: None, // TODO: Handle all of these fields.
        quoted_ttl: Some(reply.quoted_ttl),
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
