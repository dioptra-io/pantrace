use crate::atlas::models::{
    AtlasIcmpExt, AtlasIcmpExtMplsData, AtlasIcmpExtObj, AtlasTraceroute, AtlasTracerouteHop,
    AtlasTracerouteReply,
};
use crate::internal::models::{MplsEntry, TracerouteReply};
use crate::utils::protocol_string;
use std::net::IpAddr;

impl AtlasTraceroute {
    pub fn from_internal(replies: &[TracerouteReply]) -> Self {
        // TODO: assert same-flow assumption?
        let ref_reply = &replies[0];
        let start_timestamp = replies
            .iter()
            .map(|reply| reply.capture_timestamp)
            .min()
            .unwrap();
        let end_timestamp = replies
            .iter()
            .map(|reply| reply.capture_timestamp)
            .max()
            .unwrap();
        AtlasTraceroute {
            af: ref_reply.af(),
            dst_addr: Some(IpAddr::from(ref_reply.probe_dst_addr)),
            dst_name: ref_reply.probe_dst_addr.to_string(),
            endtime: end_timestamp,
            from: Some(IpAddr::from(ref_reply.probe_src_addr)),
            msm_id: 0, // TODO
            msm_name: "TODO".to_string(),
            paris_id: ref_reply.probe_src_port,
            prb_id: 0, // TODO
            proto: protocol_string(ref_reply.probe_protocol),
            result: replies
                .group_by(|a, b| a.probe_ttl == b.probe_ttl)
                .map(AtlasTracerouteHop::from_internal)
                .collect(),
            size: 0, // TODO
            src_addr: Some(IpAddr::from(ref_reply.probe_src_addr)),
            timestamp: start_timestamp,
            kind: "traceroute".to_string(),
        }
    }
}

impl AtlasTracerouteHop {
    pub fn from_internal(replies: &[TracerouteReply]) -> Self {
        // TODO: assert same-hop assumption?
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
            from: Some(IpAddr::from(reply.reply_src_addr)),
            rtt: (reply.rtt as f64) / 10.0,
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