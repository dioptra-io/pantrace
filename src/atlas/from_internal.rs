use std::ops::Deref;

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

impl From<&Traceroute> for Vec<AtlasTraceroute> {
    /// Build an AtlasTraceroute from an array of TracerouteReply.
    /// There must be at-least one reply, and all replies must have the same flow identifier.
    fn from(traceroute: &Traceroute) -> Vec<AtlasTraceroute> {
        traceroute
            .flows
            .iter()
            .map(|flow| {
                AtlasTraceroute {
                    af: traceroute.af(),
                    dst_addr: Some(traceroute.dst_addr.to_canonical()),
                    dst_name: traceroute.dst_addr.to_string(),
                    endtime: traceroute.end_time,
                    from: Some(traceroute.src_addr.to_canonical()),
                    msm_id: traceroute.measurement_id_int(),
                    msm_name: traceroute.measurement_id.clone(),
                    paris_id: flow.src_port,
                    prb_id: traceroute.agent_id_int(),
                    proto: PROTOCOL_TO_STRING[&traceroute.protocol].to_string(),
                    result: flow
                        .replies
                        .group_by(|a, b| a.probe_ttl == b.probe_ttl)
                        .map(|replies| replies.into())
                        .collect(),
                    size: 0, // TODO: size of the *probe*.
                    src_addr: Some(traceroute.src_addr.to_canonical()),
                    timestamp: traceroute.start_time,
                    kind: "traceroute".to_string(),
                }
            })
            .collect()
    }
}

impl From<&[TracerouteReply]> for AtlasTracerouteHop {
    fn from(replies: &[TracerouteReply]) -> Self {
        // TODO: assert that all replies are for the same hop?
        let ref_reply = &replies[0];
        AtlasTracerouteHop {
            hop: ref_reply.probe_ttl,
            error: None,
            result: replies.iter().map(|reply| reply.into()).collect(),
        }
    }
}

impl From<&TracerouteReply> for AtlasTracerouteReply {
    fn from(reply: &TracerouteReply) -> Self {
        AtlasTracerouteReply {
            from: Some(reply.addr.to_canonical()),
            rtt: reply.rtt,
            size: reply.size,
            ttl: reply.ttl,
            icmpext: vec![reply.mpls_labels.deref().into()],
        }
    }
}

impl From<&[MplsEntry]> for AtlasIcmpExt {
    fn from(entries: &[MplsEntry]) -> Self {
        // TODO: Store RFC4844 information.
        AtlasIcmpExt {
            version: 1,
            rfc4884: 1,
            obj: vec![entries.into()],
        }
    }
}

impl From<&[MplsEntry]> for AtlasIcmpExtObj {
    fn from(entries: &[MplsEntry]) -> Self {
        AtlasIcmpExtObj {
            class: 1,
            kind: 1,
            mpls: entries.iter().map(|entry| entry.into()).collect(),
        }
    }
}

impl From<&MplsEntry> for AtlasIcmpExtMplsData {
    fn from(entry: &MplsEntry) -> Self {
        AtlasIcmpExtMplsData {
            label: entry.label,
            exp: entry.exp,
            s: entry.bottom_of_stack,
            ttl: entry.ttl,
        }
    }
}
