use crate::internal::models::{MplsEntry, TracerouteReply};
use crate::iris::models::{IrisMplsEntry, IrisReply, IrisTraceroute};

impl IrisTraceroute {
    pub fn from_internal(replies: &[TracerouteReply]) -> Self {
        IrisTraceroute {
            probe_protocol: replies[0].probe_protocol,
            probe_src_addr: replies[0].probe_src_addr,
            probe_dst_addr: replies[0].probe_dst_addr,
            probe_src_port: replies[0].probe_src_port,
            probe_dst_port: replies[0].probe_dst_port,
            replies: replies.iter().map(IrisReply::from_internal).collect(),
        }
    }
}

impl IrisReply {
    pub fn from_internal(reply: &TracerouteReply) -> Self {
        IrisReply(
            reply.capture_timestamp,
            reply.probe_ttl,
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
