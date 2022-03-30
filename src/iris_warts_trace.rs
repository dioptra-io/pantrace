use warts::{Address, Flags, Object, TraceProbe, Traceroute, TraceStopReason, TraceType};
use crate::{IrisReply, IrisTraceroute};

impl IrisTraceroute {
    pub fn to_warts_trace(&self) -> Traceroute {
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
            src_port: Some(self.probe_src_port),
            dst_port: Some(self.probe_dst_port),
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
            src_addr: Some(Address::from(self.probe_src_addr)),
            dst_addr: Some(Address::from(self.probe_dst_addr)),
            user_id: None,
            ip_offset: None,
            router_addr: None,
            hop_count: self.replies.len() as u16,
            hops: self.replies.iter().map(|reply| reply.to_warts_trace_probe()).collect(),
            eof: 0,
        };
        t.fixup();
        return t;
    }
}

impl IrisReply {
    pub fn to_warts_trace_probe(&self) -> TraceProbe {
        let mut tp = TraceProbe {
            flags: Default::default(),
            param_length: None,
            addr_id: None,
            probe_ttl: Some(self.1),
            reply_ttl: Some(self.2),
            hop_flags: None, // TODO
            probe_id: None,
            rtt_usec: Some((self.6 * 1000.0) as u32),
            icmp_type: Some(11),
            icmp_code: Some(0),
            probe_size: None,
            reply_size: Some(self.3),
            reply_ip_id: None,
            reply_ip_tos: None,
            next_hop_mtu: None,
            quoted_length: None,
            quoted_ttl: None, // TODO: Add all these fields.
            reply_tcp_flags: None,
            quoted_tos: None,
            icmp_extensions_length: None,
            icmp_extensions: vec![], // TODO
            addr: Some(Address::from(self.5)),
            tx: None, // TODO: Substract rtt from capture_timestamp
        };
        tp.fixup();
        return tp;
    }
}