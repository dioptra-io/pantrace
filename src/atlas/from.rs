use crate::atlas::{
    AtlasIcmpExt,
    AtlasIcmpExtMplsData,
    AtlasIcmpExtObj,
    AtlasTraceroute,
    AtlasTracerouteHop,
    AtlasTracerouteReply,
};
use crate::internal::{MplsEntry, Traceroute, TracerouteReply};
use crate::utils::PROTOCOL_TO_STRING;

// TODO: Implement From<> trait instead of from_internal.

impl AtlasTraceroute {
    /// Build an AtlasTraceroute from an array of TracerouteReply.
    /// There must be at-least one reply, and all replies must have the same flow identifier.
    pub fn from_internal(traceroute: &Traceroute) -> Vec<Self> {
        traceroute
            .flows
            .iter()
            .map(|flow| {
                AtlasTraceroute {
                    af: traceroute.af(),
                    dst_addr: Some(traceroute.probe_dst_addr.to_canonical()),
                    dst_name: traceroute.probe_dst_addr.to_string(),
                    endtime: traceroute.end_time,
                    from: Some(traceroute.probe_src_addr.to_canonical()),
                    msm_id: traceroute.measurement_id_int(),
                    msm_name: traceroute.measurement_id.clone(),
                    paris_id: flow.probe_src_port,
                    prb_id: traceroute.agent_id_int(),
                    proto: PROTOCOL_TO_STRING[&traceroute.probe_protocol].to_string(),
                    result: flow
                        .replies
                        .group_by(|a, b| a.probe_ttl == b.probe_ttl)
                        .map(AtlasTracerouteHop::from_internal)
                        .collect(),
                    size: 0, // TODO: size of the *probe*.
                    src_addr: Some(traceroute.probe_src_addr.to_canonical()),
                    timestamp: traceroute.start_time,
                    kind: "traceroute".to_string(),
                }
            })
            .collect()
    }
}

impl AtlasTracerouteHop {
    pub fn from_internal(replies: &[TracerouteReply]) -> Self {
        // TODO: assert that all replies are for the same hop?
        let ref_reply = &replies[0];
        AtlasTracerouteHop {
            hop: ref_reply.probe_ttl,
            error: None,
            result: replies
                .iter()
                .map(AtlasTracerouteReply::from_internal)
                .collect(),
        }
    }
}

impl AtlasTracerouteReply {
    pub fn from_internal(reply: &TracerouteReply) -> Self {
        AtlasTracerouteReply {
            from: Some(reply.reply_src_addr.to_canonical()),
            rtt: reply.rtt_ms(),
            size: reply.reply_size,
            ttl: reply.reply_ttl,
            icmpext: vec![AtlasIcmpExt::from_internal(&reply.reply_mpls_labels)],
        }
    }
}

impl AtlasIcmpExt {
    pub fn from_internal(entries: &[MplsEntry]) -> Self {
        // TODO: Store RFC4844 information.
        AtlasIcmpExt {
            version: 1,
            rfc4884: 1,
            obj: vec![AtlasIcmpExtObj::from_internal(entries)],
        }
    }
}

impl AtlasIcmpExtObj {
    pub fn from_internal(entries: &[MplsEntry]) -> Self {
        AtlasIcmpExtObj {
            class: 1,
            kind: 1,
            mpls: entries
                .iter()
                .map(AtlasIcmpExtMplsData::from_internal)
                .collect(),
        }
    }
}

impl AtlasIcmpExtMplsData {
    pub fn from_internal(entry: &MplsEntry) -> Self {
        AtlasIcmpExtMplsData {
            label: entry.label,
            exp: entry.exp,
            s: entry.bottom_of_stack,
            ttl: entry.ttl,
        }
    }
}
