use chrono::{TimeZone, Utc};
use pantrace::atlas::models::AtlasTraceroute;
use pantrace::internal::models::TracerouteReply;
use pantrace::iris::models::IrisTraceroute;
use pantrace::warts_trace::from::warts_trace_from_internal;
use pantrace::warts_trace::to::warts_trace_to_internal;
use std::net::Ipv6Addr;
use std::str::FromStr;

fn test_replies() -> Vec<TracerouteReply> {
    let mut replies = Vec::new();
    replies.push(TracerouteReply {
        measurement_id: "1234".to_string(),
        agent_id: "5678".to_string(),
        traceroute_start: Utc.ymd(2022, 2, 1).and_hms(12, 23, 34),
        probe_protocol: 1,
        probe_src_addr: Ipv6Addr::from_str("2001:db8::1").unwrap(),
        probe_dst_addr: Ipv6Addr::from_str("2001:db8::1").unwrap(),
        probe_src_port: 24000,
        probe_dst_port: 33434,
        capture_timestamp: Utc.ymd(2022, 2, 1).and_hms(12, 23, 35),
        probe_ttl: 8,
        reply_ttl: 32,
        reply_size: 42,
        reply_mpls_labels: vec![],
        reply_src_addr: Ipv6Addr::from_str("2001:db8::3").unwrap(),
        reply_icmp_type: 11,
        reply_icmp_code: 0,
        rtt: 238,
    });
    replies
}

#[test]
fn test_atlas() {
    let before = test_replies();
    let after = AtlasTraceroute::from_internal(&before)
        .unwrap()
        .to_internal();
    // assert_eq!(before, after);
}

#[test]
fn test_iris() {
    let before = test_replies();
    let after = IrisTraceroute::from_internal(&before).to_internal();
    // assert_eq!(before, after);
}

#[test]
fn test_warts_trace() {
    let before = test_replies();
    let after = warts_trace_to_internal(&warts_trace_from_internal(&before), 1234, "5678");
    // assert_eq!(before, after);
}
