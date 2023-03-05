use crate::formats::internal::{
    MplsEntry, Traceroute, TracerouteFlow, TracerouteHop, TracerouteProbe, TracerouteReply,
};
use crate::formats::iris::{IrisFlow, IrisMplsEntry, IrisReply, IrisTraceroute};
use chrono::Duration;
use std::collections::HashMap;
use std::ops::Sub;

impl From<&IrisTraceroute> for Traceroute {
    fn from(traceroute: &IrisTraceroute) -> Traceroute {
        Traceroute {
            measurement_name: "".to_string(),
            measurement_id: traceroute.measurement_uuid.to_string(),
            agent_id: traceroute.agent_uuid.to_string(),
            start_time: traceroute.traceroute_start,
            end_time: traceroute.traceroute_end,
            protocol: traceroute.probe_protocol.try_into().unwrap(),
            src_addr: traceroute.probe_src_addr,
            src_addr_public: None,
            dst_addr: traceroute.probe_dst_addr,
            flows: traceroute.flows.iter().map(|flow| flow.into()).collect(),
        }
    }
}

impl From<&IrisFlow> for TracerouteFlow {
    fn from(flow: &IrisFlow) -> TracerouteFlow {
        let mut replies_by_ttl = HashMap::new();
        for reply in &flow.replies {
            replies_by_ttl
                .entry(reply.1)
                .or_insert_with(Vec::new)
                .push(reply);
        }
        TracerouteFlow {
            src_port: flow.probe_src_port,
            dst_port: flow.probe_dst_port,
            hops: replies_by_ttl
                .into_iter()
                .map(|(ttl, replies)| TracerouteHop {
                    ttl,
                    probes: replies.into_iter().map(|reply| reply.into()).collect(),
                })
                .collect(),
        }
    }
}

impl From<&IrisReply> for TracerouteProbe {
    fn from(reply: &IrisReply) -> TracerouteProbe {
        TracerouteProbe {
            timestamp: reply.0.sub(Duration::microseconds(reply.9 as i64 * 100)),
            size: 0,
            reply: Some(TracerouteReply {
                timestamp: reply.0,
                quoted_ttl: reply.2,
                icmp_type: reply.3,
                icmp_code: reply.4,
                ttl: reply.5,
                size: reply.6,
                mpls_labels: reply.7.iter().map(|entry| entry.into()).collect(),
                addr: reply.8,
                rtt: reply.9 as f64 / 10.0,
            }),
        }
    }
}

impl From<&IrisMplsEntry> for MplsEntry {
    fn from(entry: &IrisMplsEntry) -> MplsEntry {
        MplsEntry {
            label: entry.0,
            exp: entry.1,
            bottom_of_stack: entry.2,
            ttl: entry.3,
        }
    }
}
