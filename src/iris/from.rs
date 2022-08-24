use crate::internal::{MplsEntry, TracerouteReply};
use crate::iris::models::{IrisMplsEntry, IrisReply, IrisTraceroute};

impl IrisTraceroute {
    /// Build an [IrisTraceroute] from an array of [TracerouteReply].
    /// There must be at-least one reply, and all replies must have the same flow identifier.
    pub fn from_internal(replies: &[TracerouteReply]) -> Self {
        let ref_reply = &replies[0];
        IrisTraceroute {
            measurement_uuid: ref_reply.measurement_id.clone(),
            agent_uuid: ref_reply.agent_id.clone(),
            traceroute_start: ref_reply.traceroute_start,
            probe_protocol: ref_reply.probe_protocol,
            probe_src_addr: ref_reply.probe_src_addr,
            probe_dst_addr: ref_reply.probe_dst_addr,
            probe_src_port: ref_reply.probe_src_port,
            probe_dst_port: ref_reply.probe_dst_port,
            replies: replies.iter().map(IrisReply::from_internal).collect(),
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
            reply.rtt,
        )
    }
}

impl IrisMplsEntry {
    pub fn from_internal(entry: &MplsEntry) -> Self {
        IrisMplsEntry(entry.label, entry.exp, entry.bottom_of_stack, entry.ttl)
    }
}
