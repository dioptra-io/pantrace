use crate::formats::internal::{MplsEntry, Traceroute, TracerouteReply};
use crate::formats::iris::{IrisFlow, IrisMplsEntry, IrisReply, IrisTraceroute};

// TODO: Update docstrings (TracerouteReply -> Traceroute).
// TODO: Update Iris query for multiple flows.

impl From<&Traceroute> for IrisTraceroute {
    /// Build an [IrisTraceroute] from an array of [TracerouteReply].
    /// There must be at-least one reply, and all replies must have the same flow identifier.
    fn from(traceroute: &Traceroute) -> Self {
        IrisTraceroute {
            measurement_uuid: traceroute.measurement_id.clone(),
            agent_uuid: traceroute.agent_id.clone(),
            traceroute_start: traceroute.start_time,
            traceroute_end: traceroute.end_time,
            probe_protocol: traceroute.protocol as u8,
            probe_src_addr: traceroute.src_addr,
            probe_dst_addr: traceroute.dst_addr,
            flows: traceroute
                .flows
                .iter()
                .map(|flow| IrisFlow {
                    probe_src_port: flow.src_port,
                    probe_dst_port: flow.dst_port,
                    replies: flow.replies.iter().map(|reply| reply.into()).collect(),
                })
                .collect(),
        }
    }
}

impl From<&TracerouteReply> for IrisReply {
    fn from(reply: &TracerouteReply) -> Self {
        IrisReply(
            reply.timestamp,
            reply.probe_ttl,
            reply.quoted_ttl,
            reply.icmp_type,
            reply.icmp_code,
            reply.ttl,
            reply.size,
            reply.mpls_labels.iter().map(|entry| entry.into()).collect(),
            reply.addr,
            (reply.rtt * 10.0) as u16,
        )
    }
}

impl From<&MplsEntry> for IrisMplsEntry {
    fn from(entry: &MplsEntry) -> Self {
        IrisMplsEntry(entry.label, entry.exp, entry.bottom_of_stack, entry.ttl)
    }
}
