use std::net::IpAddr;

use crate::formats::internal::MplsEntry;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FlatTracerouteReply {
    pub measurement_id: String,
    pub agent_id: String,
    pub traceroute_start: DateTime<Utc>,
    pub probe_protocol: u8,
    pub probe_src_addr: IpAddr,
    pub probe_dst_addr: IpAddr,
    pub probe_src_port: u16,
    pub probe_dst_port: u16,
    pub capture_timestamp: DateTime<Utc>,
    pub probe_ttl: u8,
    pub quoted_ttl: u8,
    pub reply_ttl: u8,
    pub reply_size: u16,
    pub reply_mpls_labels: Vec<MplsEntry>,
    pub reply_src_addr: IpAddr,
    pub reply_icmp_type: u8,
    pub reply_icmp_code: u8,
    pub rtt: u16,
}
