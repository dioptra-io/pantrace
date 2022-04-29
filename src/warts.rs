use crate::convertable::Convertable;
use crate::TracerouteReply;
use warts::{Address, TraceProbe, TraceStopReason, TraceType, Traceroute};

impl Convertable for Traceroute {
    fn from_internal(replies: &[TracerouteReply]) -> Option<Self>
    where
        Self: Sized,
    {
        if replies.is_empty() {
            None
        } else {
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
            Some(t)
        }
    }

    fn to_internal(&self) -> Vec<TracerouteReply> {
        todo!()
    }
}

fn warts_trace_probe_from_internal(reply: &TracerouteReply) -> TraceProbe {
    let mut tp = TraceProbe {
        flags: Default::default(),
        param_length: None,
        addr_id: None,
        probe_ttl: Some(reply.probe_ttl),
        reply_ttl: Some(reply.reply_ttl),
        hop_flags: None, // TODO
        probe_id: None,
        rtt_usec: Some((reply.rtt * 1000.0) as u32),
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
        tx: None, // TODO: Substract rtt from capture_timestamp
    };
    tp.fixup();
    tp
}
