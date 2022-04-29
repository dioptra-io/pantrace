use crate::convertable::PantraceFormat;
use crate::{MplsEntry, TracerouteReply};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::Ipv6Addr;

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasTraceroute {
    pub af: u8,
    pub dst_addr: Ipv6Addr,
    pub dst_name: String,
    #[serde(with = "ts_seconds")]
    pub endtime: DateTime<Utc>,
    pub from: Ipv6Addr,
    pub msm_id: u64,
    pub msm_name: String,
    pub paris_id: u16,
    pub prb_id: u64,
    pub proto: String,
    pub result: Vec<AtlasTracerouteHop>,
    pub size: u16,
    pub src_addr: Ipv6Addr,
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
    pub from: Ipv6Addr,
    pub rtt: f64,
    pub size: u16,
    pub ttl: u8,
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
    fn from_internal(replies: &[TracerouteReply]) -> Option<Self> {
        // TODO: Assert same-flow assumption.
        if replies.is_empty() {
            None
        } else {
            Some(AtlasTraceroute {
                af: replies[0].af(),
                dst_addr: replies[0].probe_dst_addr,
                dst_name: "".to_string(),
                endtime: Utc::now(), // TODO
                from: replies[0].probe_src_addr,
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
                src_addr: replies[0].probe_src_addr,
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
    fn to_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
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
        src_addr: Ipv6Addr,
        dst_addr: Ipv6Addr,
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
            from: reply.reply_src_addr,
            rtt: reply.rtt,
            size: reply.reply_size,
            ttl: reply.probe_ttl,
            icmpext: vec![AtlasIcmpExt::from_internal(&reply.mpls_labels)],
        }
    }
    pub fn to_internal(
        &self,
        proto: &str,
        src_addr: Ipv6Addr,
        dst_addr: Ipv6Addr,
    ) -> TracerouteReply {
        // TODO: const hashmap?
        let protocols = HashMap::from([("icmp", 1), ("udp", 17), ("icmp6", 58)]);
        TracerouteReply {
            probe_protocol: protocols[proto],
            probe_src_addr: src_addr,
            probe_dst_addr: dst_addr,
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
            reply_src_addr: self.from,
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
