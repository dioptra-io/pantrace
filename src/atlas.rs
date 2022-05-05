use crate::utils::{
    default_ipaddr, empty_string_as_none, id_from_string, ipv6_from_ip, protocol_number,
    protocol_string,
};
use crate::{MplsEntry, TracerouteReply};
use chrono::serde::ts_seconds;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, Lines};
use std::net::{IpAddr, Ipv6Addr};

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasTraceroute {
    pub af: u8,
    pub dst_addr: Option<IpAddr>,
    pub dst_name: String,
    #[serde(with = "ts_seconds")]
    pub endtime: DateTime<Utc>,
    #[serde(default = "default_ipaddr", deserialize_with = "empty_string_as_none")]
    pub from: Option<IpAddr>,
    pub msm_id: u64,
    pub msm_name: String,
    #[serde(default)]
    pub paris_id: u16,
    pub prb_id: u64,
    pub proto: String,
    pub result: Vec<AtlasTracerouteHop>,
    pub size: u16,
    #[serde(default = "default_ipaddr", deserialize_with = "empty_string_as_none")]
    pub src_addr: Option<IpAddr>,
    #[serde(with = "ts_seconds")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AtlasTracerouteHop {
    #[serde(default)]
    pub hop: u8,
    pub error: Option<String>,
    #[serde(default)]
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
    #[serde(skip)]
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

pub struct AtlasReader<R: BufRead> {
    lines: Lines<R>,
}

impl<R: BufRead> AtlasReader<R> {
    pub fn new(input: R) -> AtlasReader<R> {
        AtlasReader {
            lines: input.lines(),
        }
    }
}

impl<R: BufRead> Iterator for AtlasReader<R> {
    type Item = Vec<TracerouteReply>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(line)) => serde_json::from_str::<AtlasTraceroute>(&line)
                .map(|t| Some(t.to_internal()))
                .unwrap_or(None),
            Some(Err(_)) => None,
            None => None,
        }
    }
}

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
            src_addr: Some(IpAddr::from(ref_reply.probe_src_addr)),
            timestamp: start_timestamp,
            kind: "traceroute".to_string(),
        }
    }
    pub fn to_internal(&self) -> Vec<TracerouteReply> {
        self.result
            .iter()
            .flat_map(|result| {
                result.to_internal(
                    self.msm_id,
                    self.prb_id,
                    &self.proto,
                    self.src_addr.unwrap_or(IpAddr::V6(Ipv6Addr::UNSPECIFIED)),
                    self.dst_addr.unwrap_or(IpAddr::V6(Ipv6Addr::UNSPECIFIED)),
                    self.paris_id,
                )
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
            error: None,
            result: replies
                .iter()
                .map(AtlasTracerouteReply::from_internal)
                .collect(),
        }
    }
    pub fn to_internal(
        &self,
        msm_id: u64,
        prb_id: u64,
        proto: &str,
        src_addr: IpAddr,
        dst_addr: IpAddr,
        paris_id: u16,
    ) -> Vec<TracerouteReply> {
        self.result
            .iter()
            .map(|result| {
                result.to_internal(
                    msm_id, prb_id, proto, src_addr, dst_addr, paris_id, self.hop,
                )
            })
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
            icmpext: vec![AtlasIcmpExt::from_internal(&reply.reply_mpls_labels)],
        }
    }
    pub fn to_internal(
        &self,
        msm_id: u64,
        prb_id: u64,
        proto: &str,
        src_addr: IpAddr,
        dst_addr: IpAddr,
        paris_id: u16,
        hop: u8,
    ) -> TracerouteReply {
        TracerouteReply {
            measurement_id: msm_id,
            agent_id: prb_id,
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
