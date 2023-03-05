use crate::formats::flat::FlatTracerouteReply;
use crate::formats::internal::Traceroute;

impl From<&Traceroute> for Vec<FlatTracerouteReply> {
    fn from(traceroute: &Traceroute) -> Self {
        let mut traceroutes = Vec::new();
        for flow in &traceroute.flows {
            for hop in &flow.hops {
                for probe in &hop.probes {
                    if probe.reply.is_none() {
                        continue;
                    }
                    let reply = probe.reply.as_ref().unwrap();
                    traceroutes.push(FlatTracerouteReply {
                        measurement_id: traceroute.measurement_id.to_string(),
                        agent_id: traceroute.agent_id.to_string(),
                        traceroute_start: traceroute.start_time,
                        probe_protocol: traceroute.protocol as u8,
                        probe_src_addr: traceroute.src_addr,
                        probe_dst_addr: traceroute.dst_addr,
                        probe_src_port: flow.src_port,
                        probe_dst_port: flow.dst_port,
                        capture_timestamp: reply.timestamp,
                        probe_ttl: hop.ttl,
                        quoted_ttl: reply.quoted_ttl,
                        reply_ttl: reply.ttl,
                        reply_size: reply.size,
                        reply_mpls_labels: reply.mpls_labels.clone(),
                        reply_src_addr: reply.addr,
                        reply_icmp_type: reply.icmp_type,
                        reply_icmp_code: reply.icmp_code,
                        rtt: (reply.rtt * 10.0) as u16,
                    })
                }
            }
        }
        traceroutes
    }
}
