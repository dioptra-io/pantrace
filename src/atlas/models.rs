use std::net::IpAddr;

use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::{default_ipaddr, empty_string_as_none};

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
