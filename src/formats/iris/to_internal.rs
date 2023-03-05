use crate::formats::internal::{MplsEntry, Traceroute, TracerouteFlow, TracerouteReply};
use crate::formats::iris::{IrisFlow, IrisMplsEntry, IrisReply, IrisTraceroute};

impl From<&IrisTraceroute> for Traceroute {
    fn from(traceroute: &IrisTraceroute) -> Traceroute {
        Traceroute {
            measurement_id: traceroute.measurement_uuid.to_string(),
            agent_id: traceroute.agent_uuid.to_string(),
            start_time: traceroute.traceroute_start,
            end_time: traceroute.traceroute_end,
            protocol: traceroute.probe_protocol.try_into().unwrap(),
            src_addr: traceroute.probe_src_addr,
            dst_addr: traceroute.probe_dst_addr,
            flows: traceroute.flows.iter().map(|flow| flow.into()).collect(),
        }
    }
}

impl From<&IrisFlow> for TracerouteFlow {
    fn from(flow: &IrisFlow) -> Self {
        TracerouteFlow {
            src_port: flow.probe_src_port,
            dst_port: flow.probe_dst_port,
            replies: flow.replies.iter().map(|reply| reply.into()).collect(),
        }
    }
}

impl From<&IrisReply> for TracerouteReply {
    fn from(reply: &IrisReply) -> TracerouteReply {
        TracerouteReply {
            timestamp: reply.0,
            probe_ttl: reply.1,
            quoted_ttl: reply.2,
            icmp_type: reply.3,
            icmp_code: reply.4,
            ttl: reply.5,
            size: reply.6,
            mpls_labels: reply.7.iter().map(|entry| entry.into()).collect(),
            addr: reply.8,
            rtt: reply.9 as f64 / 10.0,
        }
    }
}

impl From<&IrisMplsEntry> for MplsEntry {
    fn from(entry: &IrisMplsEntry) -> MplsEntry {
        MplsEntry {
            label: entry.0,
            exp: entry.1,
            bottom_of_stack: entry.2,
            ttl: entry.3,
        }
    }
}
