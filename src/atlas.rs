use crate::format::PantraceFormat;
use crate::{MplsEntry, TracerouteReply};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, Ipv6Addr};

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasTraceroute {
    pub af: u8,
    pub dst_addr: IpAddr,
    pub dst_name: String,
    #[serde(with = "ts_seconds")]
    pub endtime: DateTime<Utc>,
    pub from: IpAddr,
    pub msm_id: u64,
    pub msm_name: String,
    pub paris_id: u16,
    pub prb_id: u64,
    pub proto: String,
    pub result: Vec<AtlasTracerouteHop>,
    pub size: u16,
    pub src_addr: IpAddr,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasTracerouteHop {
    pub hop: u8,
    pub result: Vec<AtlasTracerouteReply>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasTracerouteReply {
    pub from: Option<IpAddr>,
    #[serde(default)]
    pub rtt: f64,
    #[serde(default)]
    pub size: u16,
    #[serde(default)]
    pub ttl: u8,
    #[serde(default)]
    pub icmpext: Vec<AtlasIcmpExt>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasIcmpExt {
    pub version: u8,
    pub rfc4884: u8,
    pub obj: Vec<AtlasIcmpExtObj>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasIcmpExtObj {
    pub class: u8,
    #[serde(rename = "type")]
    pub kind: u8,
    pub mpls: Vec<AtlasIcmpExtMplsData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasIcmpExtMplsData {
    pub label: u32,
    pub exp: u8,
    pub s: u8,
    pub ttl: u8,
}

impl PantraceFormat for AtlasTraceroute {
    fn from_bytes(data: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        Some(serde_json::from_slice(data).unwrap())
    }
    fn to_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
    fn from_internal(replies: &[TracerouteReply]) -> Option<Self> {
        // TODO: Assert same-flow assumption.
        if replies.is_empty() {
            None
        } else {
            Some(AtlasTraceroute {
                af: replies[0].af(),
                dst_addr: IpAddr::from(replies[0].probe_dst_addr),
                dst_name: "".to_string(),
                endtime: Utc::now(), // TODO
                from: IpAddr::from(replies[0].probe_src_addr),
                msm_id: 0,
                msm_name: "".to_string(),
                paris_id: 0,
                prb_id: 0,
                proto: "".to_string(),
                result: replies
                    .group_by(|a, b| a.probe_ttl == b.probe_ttl)
                    .map(AtlasTracerouteHop::from_internal)
                    .collect(),
                size: 0,
                src_addr: IpAddr::from(replies[0].probe_src_addr),
                timestamp: Utc::now(), // TODO
                kind: "".to_string(),
            })
        }
    }
    fn to_internal(&self) -> Vec<TracerouteReply> {
        self.result
            .iter()
            .flat_map(|result| result.to_internal(&self.proto, self.src_addr, self.dst_addr))
            .collect()
    }
}

impl AtlasTracerouteHop {
    pub fn from_internal(replies: &[TracerouteReply]) -> Self {
        // TODO: assert same-hop assumption?
        AtlasTracerouteHop {
            hop: 0, // TODO
            result: replies
                .iter()
                .map(AtlasTracerouteReply::from_internal)
                .collect(),
        }
    }
    pub fn to_internal(
        &self,
        proto: &str,
        src_addr: IpAddr,
        dst_addr: IpAddr,
    ) -> Vec<TracerouteReply> {
        self.result
            .iter()
            .map(|result| result.to_internal(proto, src_addr, dst_addr))
            .collect()
    }
}

impl AtlasTracerouteReply {
    pub fn from_internal(reply: &TracerouteReply) -> Self {
        AtlasTracerouteReply {
            from: Some(IpAddr::from(reply.reply_src_addr)),
            rtt: reply.rtt,
            size: reply.reply_size,
            ttl: reply.probe_ttl,
            icmpext: vec![AtlasIcmpExt::from_internal(&reply.mpls_labels)],
        }
    }
    pub fn to_internal(&self, proto: &str, src_addr: IpAddr, dst_addr: IpAddr) -> TracerouteReply {
        // TODO: const hashmap?
        let protocols = HashMap::from([("ICMP", 1), ("UDP", 17), ("ICMP6", 58)]);
        TracerouteReply {
            probe_protocol: protocols[proto],
            probe_src_addr: ipv6_from_ip(src_addr),
            probe_dst_addr: ipv6_from_ip(dst_addr),
            probe_src_port: 0,             // TODO
            probe_dst_port: 0,             // TODO
            catpure_timestamp: Utc::now(), // TODO
            probe_ttl: 0,                  // TODO
            reply_ttl: self.ttl,
            reply_size: self.size,
            mpls_labels: self
                .icmpext
                .iter()
                .flat_map(|ext| ext.to_internal())
                .collect(),
            reply_src_addr: self.from.map_or(Ipv6Addr::from(0), ipv6_from_ip),
            rtt: self.rtt,
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
    pub fn to_internal(&self) -> Vec<MplsEntry> {
        self.obj[0].to_internal()
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
    pub fn to_internal(&self) -> Vec<MplsEntry> {
        // TODO: Assert class/kind?
        self.mpls
            .iter()
            .map(AtlasIcmpExtMplsData::to_internal)
            .collect()
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
    pub fn to_internal(&self) -> MplsEntry {
        MplsEntry {
            label: self.label,
            exp: self.exp,
            bottom_of_stack: self.s,
            ttl: self.ttl,
        }
    }
}

// TODO: Move somewhere else?
fn ipv6_from_ip(addr: IpAddr) -> Ipv6Addr {
    match addr {
        IpAddr::V4(x) => x.to_ipv6_mapped(),
        IpAddr::V6(x) => x,
    }
}
