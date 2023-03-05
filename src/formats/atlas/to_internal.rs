use crate::formats::atlas::{
    AtlasIcmpExt, AtlasIcmpExtMplsData, AtlasIcmpExtObj, AtlasTraceroute, AtlasTracerouteHop,
    AtlasTracerouteReply,
};
use crate::formats::internal::{
    MplsEntry, Traceroute, TracerouteFlow, TracerouteHop, TracerouteProbe, TracerouteReply,
};
use crate::utils::UNSPECIFIED;

impl From<&AtlasTraceroute> for Traceroute {
    fn from(traceroute: &AtlasTraceroute) -> Traceroute {
        Traceroute {
            measurement_name: traceroute.msm_name.to_string(),
            measurement_id: traceroute.msm_id.to_string(),
            agent_id: traceroute.prb_id.to_string(),
            start_time: traceroute.timestamp,
            end_time: traceroute.endtime,
            protocol: traceroute.proto.parse().unwrap(),
            src_addr: traceroute.src_addr.unwrap_or(UNSPECIFIED),
            src_addr_public: traceroute.from,
            dst_addr: traceroute.dst_addr.unwrap_or(UNSPECIFIED),
            flows: vec![TracerouteFlow {
                src_port: traceroute.paris_id,
                dst_port: 0,
                hops: traceroute
                    .result
                    .iter()
                    .map(|hop| (hop, traceroute.size).into())
                    .collect(),
            }],
        }
    }
}

impl From<(&AtlasTracerouteHop, u16)> for TracerouteHop {
    fn from(hop_with_size: (&AtlasTracerouteHop, u16)) -> TracerouteHop {
        let (hop, size) = hop_with_size;
        TracerouteHop {
            ttl: hop.hop,
            probes: hop
                .result
                .iter()
                .map(|reply| (reply, size).into())
                .collect(),
        }
    }
}

impl From<(&AtlasTracerouteReply, u16)> for TracerouteProbe {
    fn from(reply_with_size: (&AtlasTracerouteReply, u16)) -> TracerouteProbe {
        let (reply, size) = reply_with_size;
        TracerouteProbe {
            timestamp: Default::default(), // Atlas does not store the send timestamp.
            size,
            reply: Some(TracerouteReply {
                timestamp: Default::default(), // Atlas does not store the capture timestamp.
                quoted_ttl: 0,                 // Atlas does not store the quoted TTL.
                ttl: reply.ttl,
                size: reply.size,
                addr: reply.from.unwrap_or(UNSPECIFIED),
                icmp_type: reply.icmp_type(),
                icmp_code: reply.icmp_code(),
                mpls_labels: reply
                    .icmpext
                    .iter()
                    .flat_map(<Vec<MplsEntry>>::from)
                    .collect(),
                rtt: reply.rtt,
            }),
        }
    }
}

impl From<&AtlasIcmpExt> for Vec<MplsEntry> {
    fn from(ext: &AtlasIcmpExt) -> Self {
        ext.obj.get(0).unwrap().into()
    }
}

impl From<&AtlasIcmpExtObj> for Vec<MplsEntry> {
    fn from(obj: &AtlasIcmpExtObj) -> Vec<MplsEntry> {
        // TODO: Assert class/kind?
        obj.mpls.iter().map(|data| data.into()).collect()
    }
}

impl From<&AtlasIcmpExtMplsData> for MplsEntry {
    fn from(data: &AtlasIcmpExtMplsData) -> MplsEntry {
        MplsEntry {
            label: data.label,
            exp: data.exp,
            bottom_of_stack: data.s,
            ttl: data.ttl,
        }
    }
}
