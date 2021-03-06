use chrono::{DateTime, Utc};
use std::net::Ipv6Addr;

// TODO: Store information about the flow ID (src-port, dst-port, ...)
#[derive(Debug, PartialEq)]
pub struct TracerouteReply {
    pub probe_protocol: u8,
    pub probe_src_addr: Ipv6Addr,
    pub probe_dst_addr: Ipv6Addr,
    pub probe_src_port: u16,
    pub probe_dst_port: u16,
    pub capture_timestamp: DateTime<Utc>,
    pub probe_ttl: u8,
    pub reply_ttl: u8,
    pub reply_size: u16,
    pub mpls_labels: Vec<MplsEntry>,
    pub reply_src_addr: Ipv6Addr,
    pub rtt: f64,
}

#[derive(Debug, PartialEq)]
pub struct MplsEntry {
    pub label: u32,
    pub exp: u8,
    pub bottom_of_stack: u8,
    pub ttl: u8,
}

impl TracerouteReply {
    pub fn af(&self) -> u8 {
        if self.probe_dst_addr.to_ipv4_mapped().is_some() {
            4
        } else {
            6
        }
    }
}
