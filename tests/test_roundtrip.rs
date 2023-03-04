use std::net::Ipv6Addr;
use std::str::FromStr;

use chrono::{TimeZone, Utc};
use pantrace::atlas::AtlasTraceroute;
use pantrace::internal::{Traceroute, TracerouteFlow, TracerouteReply};
use pantrace::iris::IrisTraceroute;
use pantrace::scamper_trace_warts::WartsTracerouteWithMeta;

fn test_traceroute() -> Traceroute {
    Traceroute {
        measurement_id: "1234".to_string(),
        agent_id: "5678".to_string(),
        start_time: Utc.with_ymd_and_hms(2022, 2, 1, 12, 23, 34).unwrap(),
        end_time: Utc.with_ymd_and_hms(2022, 2, 1, 12, 23, 36).unwrap(),
        protocol: 1,
        src_addr: Ipv6Addr::from_str("2001:db8::1").unwrap(),
        dst_addr: Ipv6Addr::from_str("2001:db8::1").unwrap(),
        flows: vec![TracerouteFlow {
            src_port: 24000,
            dst_port: 33434,
            replies: vec![TracerouteReply {
                timestamp: Utc.with_ymd_and_hms(2022, 2, 1, 12, 23, 35).unwrap(),
                probe_ttl: 8,
                quoted_ttl: 1,
                ttl: 32,
                size: 42,
                mpls_labels: vec![],
                addr: Ipv6Addr::from_str("2001:db8::3").unwrap(),
                icmp_type: 11,
                icmp_code: 0,
                rtt: 23.8,
            }],
        }],
    }
}
#[test]
fn test_atlas() {
    let internal = test_traceroute();
    let atlas: Vec<AtlasTraceroute> = (&internal).into();
    assert_eq!(atlas.len(), 1);
    assert_eq!(internal, (&atlas[0]).into());
}

#[test]
fn test_iris() {
    let internal = test_traceroute();
    let iris: IrisTraceroute = (&internal).into();
    assert_eq!(internal, (&iris).into());
}

#[test]
fn test_warts_trace() {
    let internal = test_traceroute();
    let warts: Vec<WartsTracerouteWithMeta> = (&internal).into();
    assert_eq!(warts.len(), 1);
    assert_eq!(internal, (&warts[0]).into());
}
