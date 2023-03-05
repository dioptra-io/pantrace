use std::net::IpAddr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IrisTraceroute {
    pub measurement_uuid: String,
    pub agent_uuid: String,
    pub traceroute_start: DateTime<Utc>,
    pub traceroute_end: DateTime<Utc>,
    pub probe_protocol: u8,
    pub probe_src_addr: IpAddr,
    pub probe_dst_addr: IpAddr,
    pub flows: Vec<IrisFlow>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IrisFlow {
    pub probe_src_port: u16,
    pub probe_dst_port: u16,
    pub replies: Vec<IrisReply>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IrisReply(
    /// `capture_timestamp`
    pub DateTime<Utc>,
    /// `probe_ttl`
    pub u8,
    /// `quoted_ttl`
    pub u8,
    /// `reply_icmp_type`
    pub u8,
    /// `reply_icmp_code`
    pub u8,
    /// `reply_ttl`
    pub u8,
    /// `reply_size`
    pub u16,
    /// `mpls_labels`
    pub Vec<IrisMplsEntry>,
    /// `reply_src_addr`
    pub IpAddr,
    /// `rtt`
    pub u16,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct IrisMplsEntry(
    /// `label`
    pub u32,
    /// `exp`
    pub u8,
    /// `bottom-of-stack`
    pub u8,
    /// `ttl`
    pub u8,
);
