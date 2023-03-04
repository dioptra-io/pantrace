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

impl AtlasTraceroute {
    pub fn to_internal(&self) -> Traceroute {
        Traceroute {
            measurement_id: self.msm_id.to_string(),
            agent_id: self.prb_id.to_string(),
            start_time: self.timestamp,
            end_time: self.endtime,
            probe_protocol: PROTOCOL_FROM_STRING[&self.proto],
            // TODO: Simplify this by making src/dst optional in internal model.
            probe_src_addr: ipv6_from_ip(
                self.src_addr.unwrap_or(IpAddr::from(Ipv6Addr::UNSPECIFIED)),
            ),
            probe_dst_addr: ipv6_from_ip(
                self.dst_addr.unwrap_or(IpAddr::from(Ipv6Addr::UNSPECIFIED)),
            ),
            flows: vec![TracerouteFlow {
                probe_src_port: self.paris_id,
                probe_dst_port: 0,
                replies: self
                    .result
                    .iter()
                    .flat_map(|result| result.to_internal())
                    .collect(),
            }],
        }
    }
}

impl AtlasTracerouteHop {
    pub fn to_internal(&self) -> Vec<TracerouteReply> {
        self.result
            .iter()
            .map(|result| result.to_internal(self.hop))
            .collect()
    }
}

impl AtlasTracerouteReply {
    pub fn to_internal(&self, hop: u8) -> TracerouteReply {
        TracerouteReply {
            // Atlas does not store the capture timestamp.
            capture_timestamp: Utc.timestamp_opt(0, 0).unwrap(),
            probe_ttl: hop,
            // Atlas does not store the quoted TTL.
            quoted_ttl: 0,
            reply_ttl: self.ttl,
            reply_size: self.size,
            reply_mpls_labels: self
                .icmpext
                .iter()
                .flat_map(|ext| ext.to_internal())
                .collect(),
            reply_src_addr: self.from.map_or(Ipv6Addr::from(0), ipv6_from_ip),
            reply_icmp_type: 0, // TODO: guess
            reply_icmp_code: 0, // TODO: guess
            rtt: (self.rtt * 10.0) as u16,
        }
    }
}

impl AtlasIcmpExt {
    pub fn to_internal(&self) -> Vec<MplsEntry> {
        self.obj[0].to_internal()
    }
}

impl AtlasIcmpExtObj {
    pub fn to_internal(&self) -> Vec<MplsEntry> {
        // TODO: Assert class/kind?
        self.mpls
            .iter()
            .map(AtlasIcmpExtMplsData::to_internal)
            .collect()
    }
}

impl AtlasIcmpExtMplsData {
    pub fn to_internal(&self) -> MplsEntry {
        MplsEntry {
            label: self.label,
            exp: self.exp,
            bottom_of_stack: self.s,
            ttl: self.ttl,
        }
    }
}
