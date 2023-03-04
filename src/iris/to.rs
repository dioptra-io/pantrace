use std::net::Ipv6Addr;

use chrono::{DateTime, Utc};

use crate::internal::{MplsEntry, TracerouteReply};
use crate::iris::{IrisMplsEntry, IrisReply, IrisTraceroute};

impl IrisTraceroute {
    pub fn to_internal(&self) -> Vec<TracerouteReply> {
        self.replies
            .iter()
            .map(|reply| {
                reply.to_internal(
                    &self.measurement_uuid,
                    &self.agent_uuid,
                    self.traceroute_start,
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
        measurement_uuid: &str,
        agent_uuid: &str,
        traceroute_start: DateTime<Utc>,
        probe_protocol: u8,
        probe_src_addr: Ipv6Addr,
        probe_dst_addr: Ipv6Addr,
        probe_src_port: u16,
        probe_dst_port: u16,
    ) -> TracerouteReply {
        TracerouteReply {
            measurement_id: measurement_uuid.to_owned(),
            agent_id: agent_uuid.to_owned(),
            traceroute_start,
            probe_protocol,
            probe_src_addr,
            probe_dst_addr,
            probe_src_port,
            probe_dst_port,
            capture_timestamp: self.0,
            probe_ttl: self.1,
            quoted_ttl: self.2,
            reply_icmp_type: self.3,
            reply_icmp_code: self.4,
            reply_ttl: self.5,
            reply_size: self.6,
            reply_mpls_labels: self.7.iter().map(IrisMplsEntry::to_internal).collect(),
            reply_src_addr: self.8,
            rtt: self.9,
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
