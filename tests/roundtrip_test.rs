use chrono::Utc;
use iris_converters::format::PantraceFormat;
use iris_converters::{AtlasTraceroute, IrisTraceroute, MplsEntry, TracerouteReply};
use std::net::Ipv6Addr;
use std::str::FromStr;
use warts::Traceroute;

#[test]
fn test() {
    let mut replies = Vec::new();
    replies.push(TracerouteReply {
        probe_protocol: 1,
        probe_src_addr: Ipv6Addr::from_str("2001:db8::1").unwrap(),
        probe_dst_addr: Ipv6Addr::from_str("2001:db8::2").unwrap(),
        probe_src_port: 24000,
        probe_dst_port: 33434,
        capture_timestamp: Utc::now(),
        probe_ttl: 8,
        reply_ttl: 1,
        reply_size: 16,
        mpls_labels: vec![MplsEntry {
            label: 1,
            exp: 2,
            bottom_of_stack: 3,
            ttl: 4,
        }],
        reply_src_addr: Ipv6Addr::from_str("2001:db8::3").unwrap(),
        rtt: 3.14,
    });
    assert_eq!(replies, Traceroute::from_internal(&replies).to_internal())
}
