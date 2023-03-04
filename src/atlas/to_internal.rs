use std::net::{IpAddr, Ipv6Addr};

use chrono::{TimeZone, Utc};

use crate::atlas::{
    AtlasIcmpExt,
    AtlasIcmpExtMplsData,
    AtlasIcmpExtObj,
    AtlasTraceroute,
    AtlasTracerouteHop,
    AtlasTracerouteReply,
};
use crate::internal::{MplsEntry, Traceroute, TracerouteFlow, TracerouteReply};
use crate::utils::{ipv6_from_ip, PROTOCOL_FROM_STRING};

impl From<&AtlasTraceroute> for Traceroute {
    fn from(traceroute: &AtlasTraceroute) -> Traceroute {
        Traceroute {
            measurement_id: traceroute.msm_id.to_string(),
            agent_id: traceroute.prb_id.to_string(),
            start_time: traceroute.timestamp,
            end_time: traceroute.endtime,
            probe_protocol: PROTOCOL_FROM_STRING[&traceroute.proto],
            // TODO: Simplify this by making src/dst optional in internal model.
            probe_src_addr: ipv6_from_ip(
                traceroute
                    .src_addr
                    .unwrap_or(IpAddr::from(Ipv6Addr::UNSPECIFIED)),
            ),
            probe_dst_addr: ipv6_from_ip(
                traceroute
                    .dst_addr
                    .unwrap_or(IpAddr::from(Ipv6Addr::UNSPECIFIED)),
            ),
            flows: vec![TracerouteFlow {
                probe_src_port: traceroute.paris_id,
                probe_dst_port: 0,
                replies: traceroute
                    .result
                    .iter()
                    .flat_map(<Vec<TracerouteReply>>::from)
                    .collect(),
            }],
        }
    }
}

impl From<&AtlasTracerouteHop> for Vec<TracerouteReply> {
    fn from(hop: &AtlasTracerouteHop) -> Vec<TracerouteReply> {
        hop.result
            .iter()
            .map(|result| (result, hop.hop).into())
            .collect()
    }
}

impl From<(&AtlasTracerouteReply, u8)> for TracerouteReply {
    fn from(reply_with_hop: (&AtlasTracerouteReply, u8)) -> TracerouteReply {
        let (reply, hop) = reply_with_hop;
        TracerouteReply {
            // Atlas does not store the capture timestamp.
            capture_timestamp: Utc.timestamp_opt(0, 0).unwrap(),
            probe_ttl: hop,
            // Atlas does not store the quoted TTL.
            quoted_ttl: 0,
            reply_ttl: reply.ttl,
            reply_size: reply.size,
            reply_mpls_labels: reply
                .icmpext
                .iter()
                .flat_map(<Vec<MplsEntry>>::from)
                .collect(),
            reply_src_addr: reply.from.map_or(Ipv6Addr::from(0), ipv6_from_ip),
            reply_icmp_type: 0, // TODO: guess
            reply_icmp_code: 0, // TODO: guess
            rtt: reply.rtt,
        }
    }
}

impl From<&AtlasIcmpExt> for Vec<MplsEntry> {
    fn from(ext: &AtlasIcmpExt) -> Self {
        (&ext.obj[0]).into()
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
