use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::net::Ipv6Addr;

#[derive(Debug, Serialize, Deserialize)]
pub struct IrisTraceroute {
    pub probe_protocol: u8,
    pub probe_src_addr: Ipv6Addr,
    pub probe_dst_addr: Ipv6Addr,
    pub probe_src_port: u16,
    pub probe_dst_port: u16,
    pub replies: Vec<IrisReply>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IrisMultipathTraceroute {
    pub probe_protocol: u8,
    pub probe_src_addr: Ipv6Addr,
    pub probe_dst_prefix: Ipv6Addr,
    pub replies: Vec<IrisReply>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IrisReply(
    pub DateTime<Utc>,      // capture_timestamp
    pub u8,                 // probe_ttl
    pub u8,                 // reply_ttl
    pub u16,                // reply_size
    pub Vec<IrisMplsEntry>, // mpls_labels
    pub Ipv6Addr,           // reply_src_addr
    pub u16,                // rtt
);

#[derive(Debug, Serialize, Deserialize)]
pub struct IrisMplsEntry(
    pub u32, // label
    pub u8,  // exp
    pub u8,  // bottom-of-stack
    pub u8,  // ttl
);
