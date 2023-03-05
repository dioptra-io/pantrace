use crate::formats::internal::{MplsEntry, Traceroute, TracerouteHop};
use crate::formats::iris::{IrisFlow, IrisMplsEntry, IrisReply, IrisTraceroute};

impl From<&Traceroute> for IrisTraceroute {
    fn from(traceroute: &Traceroute) -> Self {
        IrisTraceroute {
            measurement_uuid: traceroute.measurement_id.clone(),
            agent_uuid: traceroute.agent_id.clone(),
            traceroute_start: traceroute.start_time,
            traceroute_end: traceroute.end_time,
            probe_protocol: traceroute.protocol as u8,
            probe_src_addr: traceroute.src_addr,
            probe_dst_addr: traceroute.dst_addr,
            flows: traceroute
                .flows
                .iter()
                .map(|flow| IrisFlow {
                    probe_src_port: flow.src_port,
                    probe_dst_port: flow.dst_port,
                    replies: flow.hops.iter().flat_map(<Vec<IrisReply>>::from).collect(),
                })
                .collect(),
        }
    }
}

impl From<&TracerouteHop> for Vec<IrisReply> {
    fn from(hop: &TracerouteHop) -> Self {
        hop.probes
            .iter()
            // Iris does not store probes without replies
            .filter(|probe| probe.reply.is_some())
            .map(|probe| {
                let reply = probe.reply.as_ref().unwrap();
                IrisReply(
                    reply.timestamp,
                    hop.ttl,
                    reply.quoted_ttl,
                    reply.icmp_type,
                    reply.icmp_code,
                    reply.ttl,
                    reply.size,
                    reply.mpls_labels.iter().map(|entry| entry.into()).collect(),
                    reply.addr,
                    (reply.rtt * 10.0) as u16,
                )
            })
            .collect()
    }
}

impl From<&MplsEntry> for IrisMplsEntry {
    fn from(entry: &MplsEntry) -> Self {
        IrisMplsEntry(entry.label, entry.exp, entry.bottom_of_stack, entry.ttl)
    }
}
