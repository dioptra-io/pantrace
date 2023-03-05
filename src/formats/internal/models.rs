use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use seahash::hash;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Traceroute {
    /// Platform-specific measurement description
    pub measurement_name: String,
    /// Platform-specific measurement identifier:
    /// `msm_id` on Atlas, `cycle_id` on Scamper / Ark, `measurement_uuid` on Iris, etc.
    /// The precise semantics of this field depends on the platform. In general, assume that a same
    /// measurement identifier might be used by multiple traceroutes from multiple vantage points
    /// towards multiple destinations.
    pub measurement_id: String,
    /// Platform-specific vantage point identifier.
    pub agent_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub protocol: Protocol,
    pub src_addr: IpAddr,
    pub src_addr_public: Option<IpAddr>,
    pub dst_addr: IpAddr,
    pub flows: Vec<TracerouteFlow>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TracerouteFlow {
    // TODO: Store detailed flow ID information instead of src/dst port.
    //  pub enum FlowId {
    //     ICMPChecksum(u16),
    //     TCPSourcePort(u16),
    //     UDPSourcePort(u16),
    // }
    // pub id: FlowId,
    pub src_port: u16,
    pub dst_port: u16,
    // TODO: HashMap instead?
    pub hops: Vec<TracerouteHop>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TracerouteHop {
    pub ttl: u8,
    pub probes: Vec<TracerouteProbe>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TracerouteProbe {
    pub timestamp: DateTime<Utc>,
    pub size: u16,
    pub reply: Option<TracerouteReply>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TracerouteReply {
    pub timestamp: DateTime<Utc>,
    pub quoted_ttl: u8,
    pub ttl: u8,
    pub size: u16,
    pub addr: IpAddr,
    pub icmp_type: u8,
    pub icmp_code: u8,
    pub mpls_labels: Vec<MplsEntry>,
    pub rtt: f64,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MplsEntry {
    pub label: u32,
    pub exp: u8,
    pub bottom_of_stack: u8,
    pub ttl: u8,
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
            "TCP" => Ok(Protocol::TCP),
            "UDP" => Ok(Protocol::UDP),
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
