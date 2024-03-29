use warts::{
    Address, Timeval, TraceProbe, TraceStopReason, TraceType, Traceroute as WartsTraceroute,
};

use crate::formats::internal::{Traceroute, TracerouteHop};
use crate::formats::scamper_trace_warts::models::ScamperTraceWarts;

impl From<&Traceroute> for Vec<ScamperTraceWarts> {
    fn from(traceroute: &Traceroute) -> Self {
        traceroute
            .flows
            .iter()
            .map(|flow| {
                ScamperTraceWarts {
                    cycle_id: traceroute.measurement_id_int() as u32, // TODO: proper handling / round-tripping
                    monitor_name: traceroute.agent_id.to_string(),
                    traceroute: WartsTraceroute {
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
                        hop_count: flow.hops.len() as u16,
                        hops: flow.hops.iter().flat_map(<Vec<TraceProbe>>::from).collect(),
                        eof: 0,
                    }
                    .finalize(),
                }
            })
            .collect()
    }
}

impl From<&TracerouteHop> for Vec<TraceProbe> {
    fn from(hop: &TracerouteHop) -> Self {
        hop.probes
            .iter()
            .map(|probe| {
                let reply = probe.reply.as_ref();
                TraceProbe {
                    flags: Default::default(),
                    param_length: None,
                    addr_id: None,
                    probe_ttl: Some(hop.ttl),
                    reply_ttl: reply.map(|r| r.ttl),
                    hop_flags: None, // TODO
                    probe_id: None,
                    rtt_usec: reply.map(|r| r.rtt as u32 * 100),
                    icmp_type: Some(11),
                    icmp_code: Some(0),
                    probe_size: None,
                    reply_size: reply.map(|r| r.size),
                    reply_ip_id: None,
                    reply_ip_tos: None,
                    next_hop_mtu: None,
                    quoted_length: None, // TODO: Handle all of these fields.
                    quoted_ttl: reply.map(|r| r.quoted_ttl),
                    reply_tcp_flags: None,
                    quoted_tos: None,
                    icmp_extensions_length: None,
                    icmp_extensions: vec![], // TODO
                    addr: reply.map(|r| Address::from(r.addr)),
                    tx: Some(Timeval::from(probe.timestamp.naive_utc())),
                }
                .finalize()
            })
            .collect()
    }
}
