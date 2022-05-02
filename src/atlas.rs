use crate::format::PantraceFormat;
use crate::{MplsEntry, TracerouteReply};
use chrono::serde::ts_seconds;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use sha2::Sha256;
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
    fn from_internal(replies: &[TracerouteReply]) -> Self {
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
            dst_addr: IpAddr::from(ref_reply.probe_dst_addr),
            dst_name: ref_reply.probe_dst_addr.to_string(),
            endtime: end_timestamp,
            from: IpAddr::from(ref_reply.probe_src_addr),
            msm_id: id_from_string("TODO"),
            msm_name: "TODO".to_string(),
            paris_id: ref_reply.probe_src_port,
            prb_id: id_from_string("TODO"),
            proto: protocol_string(ref_reply.probe_protocol),
            result: replies
                .group_by(|a, b| a.probe_ttl == b.probe_ttl)
                .map(AtlasTracerouteHop::from_internal)
                .collect(),
            size: 0, // TODO
            src_addr: IpAddr::from(ref_reply.probe_src_addr),
            timestamp: start_timestamp,
            kind: "traceroute".to_string(),
        }
    }
    fn to_internal(&self) -> Vec<TracerouteReply> {
        self.result
            .iter()
            .flat_map(|result| {
                result.to_internal(&self.proto, self.src_addr, self.dst_addr, self.paris_id)
            })
            .collect()
    }
}

impl AtlasTracerouteHop {
    pub fn from_internal(replies: &[TracerouteReply]) -> Self {
        // TODO: assert same-hop assumption?
        let ref_reply = &replies[0];
        AtlasTracerouteHop {
            hop: ref_reply.probe_ttl,
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
        paris_id: u16,
    ) -> Vec<TracerouteReply> {
        self.result
            .iter()
            .map(|result| result.to_internal(proto, src_addr, dst_addr, paris_id, self.hop))
            .collect()
    }
}

impl AtlasTracerouteReply {
    pub fn from_internal(reply: &TracerouteReply) -> Self {
        AtlasTracerouteReply {
            from: Some(IpAddr::from(reply.reply_src_addr)),
            rtt: reply.rtt,
            size: reply.reply_size,
            ttl: reply.reply_ttl,
            icmpext: vec![AtlasIcmpExt::from_internal(&reply.mpls_labels)],
        }
    }
    pub fn to_internal(
        &self,
        proto: &str,
        src_addr: IpAddr,
        dst_addr: IpAddr,
        paris_id: u16,
        hop: u8,
    ) -> TracerouteReply {
        TracerouteReply {
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

fn id_from_string(s: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(s);
    let result = hasher.finalize();
    u64::from_le_bytes(result.as_slice()[..8].try_into().unwrap())
}

fn protocol_number(s: &str) -> u8 {
    match s {
        "ICMP" => 1,
        "ICMP6" => 58,
        "UDP" => 17,
        _ => panic!("Unsupported protocol"),
    }
}

fn protocol_string(n: u8) -> String {
    match n {
        1 => String::from("ICMP"),
        17 => String::from("UDP"),
        58 => String::from("ICMP6"),
        _ => panic!("Unsupported protocol"),
    }
}
