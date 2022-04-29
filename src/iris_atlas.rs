use crate::{
    AtlasIcmpExt, AtlasIcmpExtMplsData, AtlasIcmpExtObj, AtlasTraceroute, AtlasTracerouteHop,
    AtlasTracerouteReply, IrisReply, IrisTraceroute,
};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

fn id_from_string(s: &str) -> u64 {
    let mut hasher = Sha256::new();
    hasher.update(s);
    let result = hasher.finalize();
    u64::from_le_bytes(result.as_slice()[..8].try_into().unwrap())
}

impl IrisTraceroute {
    pub fn to_atlas_traceroute(&self, measurement_uuid: &str, agent_uuid: &str) -> AtlasTraceroute {
        let protocols = HashMap::from([(1, "icmp"), (17, "udp"), (58, "icmp6")]);
        let start_timestamp = self.replies.iter().map(|reply| reply.0).min().unwrap();
        let end_timestamp = self.replies.iter().map(|reply| reply.0).max().unwrap();
        AtlasTraceroute {
            af: self.af(),
            dst_addr: self.probe_dst_addr,
            dst_name: self.probe_dst_addr.to_string(),
            endtime: end_timestamp,
            from: self.probe_src_addr,
            msm_id: id_from_string(measurement_uuid),
            msm_name: String::from(measurement_uuid),
            paris_id: self.probe_src_port,
            prb_id: id_from_string(agent_uuid),
            proto: protocols[&self.probe_protocol].parse().unwrap(),
            result: self
                .replies
                .iter()
                .map(|reply| reply.to_atlas_hop())
                .collect(),
            size: 0, // TODO
            src_addr: self.probe_src_addr,
            timestamp: start_timestamp,
            kind: "traceroute".to_string(),
        }
    }
}

impl IrisReply {
    pub fn to_atlas_hop(&self) -> AtlasTracerouteHop {
        AtlasTracerouteHop {
            hop: self.1,
            result: vec![self.to_atlas_reply()],
        }
    }

    pub fn to_atlas_reply(&self) -> AtlasTracerouteReply {
        let mut icmpext = vec![];
        if !self.4.is_empty() {
            let mpls = self
                .4
                .iter()
                .map(|entry| AtlasIcmpExtMplsData {
                    label: entry.0,
                    exp: entry.1,
                    s: entry.2,
                    ttl: entry.3,
                })
                .collect();
            let obj = AtlasIcmpExtObj {
                class: 1,
                kind: 1,
                mpls,
            };
            let ext = AtlasIcmpExt {
                version: 2,
                rfc4884: 1,
                obj: vec![obj],
            };
            icmpext.push(ext);
        }
        AtlasTracerouteReply {
            from: self.5,
            rtt: self.6,
            size: self.3,
            ttl: self.2,
            icmpext,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{IrisMplsEntry, IrisReply};
    use chrono::{TimeZone, Utc};
    use std::net::Ipv6Addr;
    use std::str::FromStr;

    #[test]
    fn to_atlas_reply() {
        let iris_reply = IrisReply(
            Utc.ymd(2022, 3, 1).and_hms(1, 2, 3),
            1,
            255,
            42,
            vec![IrisMplsEntry(1, 2, 3, 4)],
            Ipv6Addr::from_str("2001:db8::1").unwrap(),
            4.2,
        );
        let atlas_reply = iris_reply.to_atlas_reply();
        assert_eq!(atlas_reply.from, Ipv6Addr::from_str("2001:db8::1").unwrap());
        assert_eq!(atlas_reply.rtt, 4.2);
        assert_eq!(atlas_reply.size, 42);
        assert_eq!(atlas_reply.ttl, 255);
        assert_eq!(atlas_reply.icmpext[0].version, 2);
        assert_eq!(atlas_reply.icmpext[0].rfc4884, 1);
        assert_eq!(atlas_reply.icmpext[0].obj[0].class, 1);
        assert_eq!(atlas_reply.icmpext[0].obj[0].kind, 1);
        assert_eq!(atlas_reply.icmpext[0].obj[0].mpls[0].label, 1);
        assert_eq!(atlas_reply.icmpext[0].obj[0].mpls[0].exp, 2);
        assert_eq!(atlas_reply.icmpext[0].obj[0].mpls[0].s, 3);
        assert_eq!(atlas_reply.icmpext[0].obj[0].mpls[0].ttl, 4);
    }
}
