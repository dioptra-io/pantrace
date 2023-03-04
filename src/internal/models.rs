use std::net::Ipv6Addr;
use std::ops::Sub;

use chrono::{DateTime, Duration, Utc};
use seahash::hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Traceroute {
    /// Platform-specific measurement identifier:
    /// `msm_id` on Atlas, `cycle_id` on Scamper / Ark, `measurement_uuid` on Iris...
    /// The precise semantics of this field depends on the platform. In general, assume that a same
    /// measurement identifier might be used by multiple traceroutes from multiple vantage points
    /// towards multiple destinations.
    pub measurement_id: String,
    /// Platform-specific vantage point identifier.
    pub agent_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    // TODO: Remove probe_ and reply_ from field names?
    // TODO: Enum for this.
    pub probe_protocol: u8,
    // TODO: Make these optional to simplify some code (e.g. RIPE Atlas?)
    pub probe_src_addr: Ipv6Addr,
    pub probe_dst_addr: Ipv6Addr,
    pub flows: Vec<TracerouteFlow>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TracerouteFlow {
    // TODO: Store information about the method used to vary the flow ID (src-port, dst-port, ...)
    // TODO: Use enum/variant here to store other fields than src/dst ports?
    pub probe_src_port: u16,
    pub probe_dst_port: u16,
    pub replies: Vec<TracerouteReply>,
}

// TODO: Store platform-specific metadata in a hashmap, to allow for proper round-tripping.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TracerouteReply {
    pub capture_timestamp: DateTime<Utc>,
    pub probe_ttl: u8,
    pub quoted_ttl: u8,
    pub reply_ttl: u8,
    pub reply_size: u16,
    pub reply_mpls_labels: Vec<MplsEntry>,
    pub reply_src_addr: Ipv6Addr,
    pub reply_icmp_type: u8,
    pub reply_icmp_code: u8,
    pub rtt: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MplsEntry {
    pub label: u32,
    pub exp: u8,
    pub bottom_of_stack: u8,
    pub ttl: u8,
}

impl Traceroute {
    pub fn af(&self) -> u8 {
        if self.probe_dst_addr.to_ipv4_mapped().is_some() {
            4
        } else {
            6
        }
    }
    // Not all platform use an integer agent id.
    // Our strategy is to first try to parse the id as an int, and fall back on the hash otherwise.
    pub fn agent_id_int(&self) -> u64 {
        self.agent_id
            .parse()
            .unwrap_or_else(|_| hash(self.agent_id.as_bytes()))
    }
    // See `agent_id_int`.
    pub fn measurement_id_int(&self) -> u64 {
        self.measurement_id
            .parse()
            .unwrap_or_else(|_| hash(self.measurement_id.as_bytes()))
    }
}

impl TracerouteReply {
    pub fn send_timestamp(&self) -> DateTime<Utc> {
        self.capture_timestamp
            .sub(Duration::microseconds((self.rtt * 1000.0) as i64))
    }
}
