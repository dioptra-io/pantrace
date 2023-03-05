use std::ops::Deref;

use crate::formats::atlas::{
    AtlasIcmpExt, AtlasIcmpExtMplsData, AtlasIcmpExtObj, AtlasTraceroute, AtlasTracerouteHop,
    AtlasTracerouteReply,
};
use crate::formats::internal::{MplsEntry, Traceroute, TracerouteHop, TracerouteReply};

impl From<&Traceroute> for Vec<AtlasTraceroute> {
    fn from(traceroute: &Traceroute) -> Vec<AtlasTraceroute> {
        traceroute
            .flows
            .iter()
            .map(|flow| AtlasTraceroute {
                af: traceroute.af(),
                dst_addr: Some(traceroute.dst_addr),
                dst_name: traceroute.dst_addr.to_string(),
                endtime: traceroute.end_time,
                from: traceroute.src_addr_public,
                msm_id: traceroute.measurement_id_int(),
                msm_name: traceroute.measurement_name.to_string(),
                paris_id: flow.src_port,
                prb_id: traceroute.agent_id_int(),
                proto: traceroute.protocol.to_string(),
                result: flow.hops.iter().map(|hop| hop.into()).collect(),
                // Retrieve the size of the first probe.
                size: flow
                    .hops
                    .iter()
                    .flat_map(|hop| &hop.probes)
                    .map(|probe| probe.size)
                    .next()
                    .unwrap_or(0),
                src_addr: Some(traceroute.src_addr),
                timestamp: traceroute.start_time,
                kind: "traceroute".to_string(),
            })
            .collect()
    }
}

impl From<&TracerouteHop> for AtlasTracerouteHop {
    fn from(hop: &TracerouteHop) -> Self {
        AtlasTracerouteHop {
            hop: hop.ttl,
            error: None, // TODO: implement by looking at Destination Unreachable replies.
            result: hop
                .probes
                .iter()
                .flat_map(|probe| &probe.reply)
                .map(|reply| reply.into())
                .collect(),
        }
    }
}

impl From<&TracerouteReply> for AtlasTracerouteReply {
    fn from(reply: &TracerouteReply) -> Self {
        AtlasTracerouteReply {
            from: if reply.addr.is_unspecified() {
                None
            } else {
                Some(reply.addr)
            },
            rtt: reply.rtt,
            size: reply.size,
            ttl: reply.ttl,
            icmpext: if reply.mpls_labels.is_empty() {
                vec![]
            } else {
                vec![reply.mpls_labels.deref().into()]
            },
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
