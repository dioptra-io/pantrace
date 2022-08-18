use crate::atlas::models::{
    AtlasIcmpExt, AtlasIcmpExtMplsData, AtlasIcmpExtObj, AtlasTraceroute, AtlasTracerouteHop,
    AtlasTracerouteReply,
};
use crate::internal::models::{MplsEntry, TracerouteReply};
use crate::utils::{ipv6_from_ip, protocol_number};
use chrono::{DateTime, TimeZone, Utc};
use std::net::{IpAddr, Ipv6Addr};

impl AtlasTraceroute {
    pub fn to_internal(&self) -> Vec<TracerouteReply> {
        self.result
            .iter()
            .flat_map(|result| {
                result.to_internal(
                    self.msm_id,
                    self.prb_id,
                    self.timestamp,
                    &self.proto,
                    self.from.unwrap_or(IpAddr::V6(Ipv6Addr::UNSPECIFIED)),
                    self.dst_addr.unwrap_or(IpAddr::V6(Ipv6Addr::UNSPECIFIED)),
                    self.paris_id,
                )
            })
            .collect()
    }
}

impl AtlasTracerouteHop {
    pub fn to_internal(
        &self,
        msm_id: u64,
        prb_id: u64,
        timestamp: DateTime<Utc>,
        proto: &str,
        src_addr: IpAddr,
        dst_addr: IpAddr,
        paris_id: u16,
    ) -> Vec<TracerouteReply> {
        self.result
            .iter()
            .map(|result| {
                result.to_internal(
                    msm_id, prb_id, timestamp, proto, src_addr, dst_addr, paris_id, self.hop,
                )
            })
            .collect()
    }
}

impl AtlasTracerouteReply {
    pub fn to_internal(
        &self,
        msm_id: u64,
        prb_id: u64,
        timestamp: DateTime<Utc>,
        proto: &str,
        src_addr: IpAddr,
        dst_addr: IpAddr,
        paris_id: u16,
        hop: u8,
    ) -> TracerouteReply {
        TracerouteReply {
            measurement_id: msm_id.to_string(),
            agent_id: prb_id.to_string(),
            traceroute_start: timestamp,
            probe_protocol: protocol_number(proto),
            probe_src_addr: ipv6_from_ip(src_addr),
            probe_dst_addr: ipv6_from_ip(dst_addr),
            probe_src_port: paris_id,
            probe_dst_port: 0,
            // Atlas does not store capture timestamp.
            capture_timestamp: Utc.timestamp(0, 0),
            probe_ttl: hop,
            reply_ttl: self.ttl,
            reply_size: self.size,
            reply_mpls_labels: self
                .icmpext
                .iter()
                .flat_map(|ext| ext.to_internal())
                .collect(),
            reply_src_addr: self.from.map_or(Ipv6Addr::from(0), ipv6_from_ip),
            reply_icmp_type: 0, // TODO
            reply_icmp_code: 0, // TODO
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
