use warts::{
    Address,
    Timeval,
    TraceProbe,
    TraceStopReason,
    TraceType,
    Traceroute as WartsTraceroute,
};

use crate::internal::{Traceroute, TracerouteReply};
use crate::scamper_trace_warts::models::WartsTracerouteWithMeta;

impl From<&Traceroute> for Vec<WartsTracerouteWithMeta> {
    fn from(traceroute: &Traceroute) -> Self {
        traceroute
            .flows
            .iter()
            .map(|flow| {
                let mut t = WartsTraceroute {
                    length: 0,
                    flags: Default::default(),
                    param_length: None,
                    list_id: Some(1),  // TODO: Use value stored in traceroute.
                    cycle_id: Some(1), // TODO: Use value stored in traceroute.
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
                    src_port: Some(flow.src_port),
                    dst_port: Some(flow.dst_port),
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
                    src_addr: Some(Address::from(traceroute.src_addr)),
                    dst_addr: Some(Address::from(traceroute.dst_addr)),
                    user_id: None,
                    ip_offset: None,
                    router_addr: None,
                    hop_count: flow.replies.len() as u16,
                    hops: flow.replies.iter().map(|reply| reply.into()).collect(),
                    eof: 0,
                };
                t.fixup();
                WartsTracerouteWithMeta {
                    cycle_id: traceroute.measurement_id_int() as u32, // TODO: proper handling / round-tripping
                    monitor_name: traceroute.agent_id.to_string(),
                    traceroute: t,
                }
            })
            .collect()
    }
}

impl From<&TracerouteReply> for TraceProbe {
    fn from(reply: &TracerouteReply) -> Self {
        let mut tp = TraceProbe {
            flags: Default::default(),
            param_length: None,
            addr_id: None,
            probe_ttl: Some(reply.probe_ttl),
            reply_ttl: Some(reply.ttl),
            hop_flags: None, // TODO
            probe_id: None,
            rtt_usec: Some(reply.rtt as u32 * 100),
            icmp_type: Some(11),
            icmp_code: Some(0),
            probe_size: None,
            reply_size: Some(reply.size),
            reply_ip_id: None,
            reply_ip_tos: None,
            next_hop_mtu: None,
            quoted_length: None, // TODO: Handle all of these fields.
            quoted_ttl: Some(reply.quoted_ttl),
            reply_tcp_flags: None,
            quoted_tos: None,
            icmp_extensions_length: None,
            icmp_extensions: vec![], // TODO
            addr: Some(Address::from(reply.addr)),
            tx: Some(Timeval::from(reply.send_timestamp().naive_utc())),
        };
        tp.fixup();
        tp
    }
}
