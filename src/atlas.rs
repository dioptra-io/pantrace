use std::net::Ipv6Addr;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use chrono::serde::ts_seconds;

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
