use crate::internal::models::{MplsEntry, TracerouteReply};
use crate::iris::models::{IrisMplsEntry, IrisReply, IrisTraceroute};
use chrono::Utc;
use std::net::Ipv6Addr;

impl IrisTraceroute {
    pub fn to_internal(&self) -> Vec<TracerouteReply> {
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
    pub fn to_internal(
        &self,
        probe_protocol: u8,
        probe_src_addr: Ipv6Addr,
        probe_dst_addr: Ipv6Addr,
        probe_src_port: u16,
        probe_dst_port: u16,
    ) -> TracerouteReply {
        TracerouteReply {
            measurement_id: "".to_string(), // TODO
            agent_id: "".to_string(),       // TODO
            traceroute_start: Utc::now(),   // TODO
            probe_protocol,
            probe_src_addr,
            probe_dst_addr,
            probe_src_port,
            probe_dst_port,
            capture_timestamp: self.0,
            probe_ttl: self.1,
            reply_ttl: self.2,
            reply_size: self.3,
            reply_mpls_labels: self.4.iter().map(IrisMplsEntry::to_internal).collect(),
            reply_src_addr: self.5,
            reply_icmp_type: 0, // TODO
            reply_icmp_code: 0, // TODO
            rtt: self.6,
        }
    }
}

impl IrisMplsEntry {
    pub fn to_internal(&self) -> MplsEntry {
        MplsEntry {
            label: self.0,
            exp: self.1,
            bottom_of_stack: self.2,
            ttl: self.3,
        }
    }
}
