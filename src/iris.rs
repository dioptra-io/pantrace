use crate::format::PantraceFormat;
use crate::{MplsEntry, TracerouteReply};
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
    pub f64,                // rtt
);

#[derive(Debug, Serialize, Deserialize)]
pub struct IrisMplsEntry(
    pub u32, // label
    pub u8,  // exp
    pub u8,  // bottom-of-stack
    pub u8,  // ttl
);

impl PantraceFormat for IrisTraceroute {
    fn from_bytes(data: &[u8]) -> Option<Self>
    where
        Self: Sized,
    {
        serde_json::from_slice(data).unwrap_or(None)
    }
    fn to_bytes(self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
    fn from_internal(replies: &[TracerouteReply]) -> Self {
        IrisTraceroute {
            probe_protocol: replies[0].probe_protocol,
            probe_src_addr: replies[0].probe_src_addr,
            probe_dst_addr: replies[0].probe_dst_addr,
            probe_src_port: replies[0].probe_src_port,
            probe_dst_port: replies[0].probe_dst_port,
            replies: replies.iter().map(IrisReply::from_internal).collect(),
        }
    }

    fn to_internal(&self) -> Vec<TracerouteReply> {
        self.replies
            .iter()
            .map(|reply| {
                reply.to_internal(
                    self.probe_protocol,
                    self.probe_src_addr,
                    self.probe_dst_addr,
                    self.probe_src_port,
                    self.probe_dst_port,
                )
            })
            .collect()
    }
}

impl IrisReply {
    pub fn from_internal(reply: &TracerouteReply) -> Self {
        IrisReply(
            reply.capture_timestamp,
            reply.probe_ttl,
            reply.reply_ttl,
            reply.reply_size,
            reply
                .mpls_labels
                .iter()
                .map(IrisMplsEntry::from_internal)
                .collect(),
            reply.reply_src_addr,
            reply.rtt,
        )
    }
    pub fn to_internal(
        &self,
        probe_protocol: u8,
        probe_src_addr: Ipv6Addr,
        probe_dst_addr: Ipv6Addr,
        probe_src_port: u16,
        probe_dst_port: u16,
    ) -> TracerouteReply {
        TracerouteReply {
            probe_protocol,
            probe_src_addr,
            probe_dst_addr,
            probe_src_port,
            probe_dst_port,
            capture_timestamp: self.0,
            probe_ttl: self.1,
            reply_ttl: self.2,
            reply_size: self.3,
            mpls_labels: self.4.iter().map(IrisMplsEntry::to_internal).collect(),
            reply_src_addr: self.5,
            rtt: self.6,
        }
    }
}

impl IrisMplsEntry {
    pub fn from_internal(entry: &MplsEntry) -> Self {
        IrisMplsEntry(entry.label, entry.exp, entry.bottom_of_stack, entry.ttl)
    }
    pub fn to_internal(&self) -> MplsEntry {
        MplsEntry {
            label: self.0,
            exp: self.1,
            bottom_of_stack: self.2,
            ttl: self.3,
        }
    }
}
