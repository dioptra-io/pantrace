use crate::internal::{MplsEntry, Traceroute, TracerouteFlow, TracerouteReply};
use crate::iris::{IrisMplsEntry, IrisReply, IrisTraceroute};

// TODO: Impl To?

impl IrisTraceroute {
    pub fn to_internal(&self) -> Traceroute {
        Traceroute {
            measurement_id: self.measurement_uuid.to_string(),
            agent_id: self.agent_uuid.to_string(),
            start_time: self.traceroute_start,
            end_time: Default::default(), // TODO
            probe_protocol: self.probe_protocol,
            probe_src_addr: self.probe_src_addr,
            probe_dst_addr: self.probe_dst_addr,
            // TODO: Use From/To to simplify this.
            flows: self
                .flows
                .iter()
                .map(|flow| TracerouteFlow {
                    probe_src_port: flow.probe_src_port,
                    probe_dst_port: flow.probe_dst_port,
                    replies: flow
                        .replies
                        .iter()
                        .map(|reply| reply.to_internal())
                        .collect(),
                })
                .collect(),
        }
    }
}

impl IrisReply {
    pub fn to_internal(&self) -> TracerouteReply {
        TracerouteReply {
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
