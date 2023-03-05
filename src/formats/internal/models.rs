use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::ops::Sub;
use std::str::FromStr;

use chrono::{DateTime, Duration, Utc};
use seahash::hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    // TODO: Enum for protocol and replace phf_map?
    pub protocol: Protocol,
    pub src_addr: IpAddr,
    pub dst_addr: IpAddr,
    pub flows: Vec<TracerouteFlow>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TracerouteFlow {
    // TODO: Store information about the method used to vary the flow ID (src-port, dst-port, ...)
    // TODO: Use enum/variant here to store other fields than src/dst ports?
    pub src_port: u16,
    pub dst_port: u16,
    pub replies: Vec<TracerouteReply>,
}

// TODO: Store platform-specific metadata in a hashmap, to allow for proper round-tripping.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TracerouteReply {
    pub timestamp: DateTime<Utc>,
    pub probe_ttl: u8,
    pub quoted_ttl: u8,
    pub ttl: u8,
    pub size: u16,
    pub mpls_labels: Vec<MplsEntry>,
    pub addr: IpAddr,
    pub icmp_type: u8,
    pub icmp_code: u8,
    pub rtt: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MplsEntry {
    pub label: u32,
    pub exp: u8,
    pub bottom_of_stack: u8,
    pub ttl: u8,
}

impl Traceroute {
    pub fn af(&self) -> u8 {
        if self.dst_addr.is_ipv4() {
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

impl TracerouteFlow {
    pub fn replies_by_ttl(&self) -> HashMap<u8, Vec<TracerouteReply>> {
        let mut map = HashMap::new();
        for reply in &self.replies {
            map.entry(reply.probe_ttl)
                .or_insert_with(Vec::new)
                .push(reply.clone());
        }
        map
    }
}

impl TracerouteReply {
    pub fn send_timestamp(&self) -> DateTime<Utc> {
        self.timestamp
            .sub(Duration::microseconds((self.rtt * 1000.0) as i64))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Protocol {
    ICMP,
    ICMPv6,
    TCP,
    UDP,
}

impl Display for Protocol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::ICMP => write!(f, "ICMP"),
            Protocol::ICMPv6 => write!(f, "ICMP6"),
            Protocol::TCP => write!(f, "TCP"),
            Protocol::UDP => write!(f, "UDP"),
        }
    }
}

impl FromStr for Protocol {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ICMP" => Ok(Protocol::ICMP),
            "ICMP6" => Ok(Protocol::ICMP),
            "TCP" => Ok(Protocol::ICMP),
            "UDP" => Ok(Protocol::ICMP),
            _ => Err(format!("Unsupported protocol: {s}")),
        }
    }
}

impl TryFrom<u8> for Protocol {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Protocol::ICMP),
            6 => Ok(Protocol::TCP),
            17 => Ok(Protocol::UDP),
            58 => Ok(Protocol::ICMPv6),
            _ => Err(format!("Unsupported protocol: {value}")),
        }
    }
}
