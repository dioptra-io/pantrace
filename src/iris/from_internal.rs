use crate::internal::{MplsEntry, Traceroute, TracerouteReply};
use crate::iris::{IrisFlow, IrisMplsEntry, IrisReply, IrisTraceroute};

// TODO: Update docstrings (TracerouteReply -> Traceroute).
// TODO: Update Iris query for multiple flows.

impl IrisTraceroute {
    /// Build an [IrisTraceroute] from an array of [TracerouteReply].
    /// There must be at-least one reply, and all replies must have the same flow identifier.
    pub fn from_internal(traceroute: &Traceroute) -> Self {
        IrisTraceroute {
            measurement_uuid: traceroute.measurement_id.clone(),
            agent_uuid: traceroute.agent_id.clone(),
            traceroute_start: traceroute.start_time,
            probe_protocol: traceroute.probe_protocol,
            probe_src_addr: traceroute.probe_src_addr,
            probe_dst_addr: traceroute.probe_dst_addr,
            flows: traceroute
                .flows
                .iter()
                .map(|flow| IrisFlow {
                    probe_src_port: flow.probe_src_port,
                    probe_dst_port: flow.probe_dst_port,
                    replies: flow.replies.iter().map(IrisReply::from_internal).collect(),
                })
                .collect(),
        }
    }
}

impl IrisReply {
    pub fn from_internal(reply: &TracerouteReply) -> Self {
        IrisReply(
            reply.capture_timestamp,
            reply.probe_ttl,
            reply.quoted_ttl,
            reply.reply_icmp_type,
            reply.reply_icmp_code,
            reply.reply_ttl,
            reply.reply_size,
            reply
                .reply_mpls_labels
                .iter()
                .map(IrisMplsEntry::from_internal)
                .collect(),
            reply.reply_src_addr,
            (reply.rtt * 10.0) as u16,
        )
    }
}

impl IrisMplsEntry {
    pub fn from_internal(entry: &MplsEntry) -> Self {
        IrisMplsEntry(entry.label, entry.exp, entry.bottom_of_stack, entry.ttl)
    }
}
