/* use chrono::Utc; */
/* use pantrace::internal::models::{MplsEntry, TracerouteReply}; */
/* use pantrace::iris::models::IrisTraceroute; */
/* use std::net::Ipv6Addr; */
/* use std::str::FromStr; */
/*  */
/* #[test] */
/* fn test_iris() { */
/*     let mut replies = Vec::new(); */
/*     replies.push(TracerouteReply { */
/*         measurement_id: 0, */
/*         agent_id: 0, */
/*         measurement_start: Utc::now(), */
/*         probe_protocol: 1, */
/*         probe_src_addr: Ipv6Addr::from_str("2001:db8::1").unwrap(), */
/*         probe_dst_addr: Ipv6Addr::from_str("2001:db8::2").unwrap(), */
/*         probe_src_port: 24000, */
/*         probe_dst_port: 33434, */
/*         capture_timestamp: Utc::now(), */
/*         probe_ttl: 8, */
/*         reply_ttl: 1, */
/*         reply_size: 16, */
/*         reply_mpls_labels: vec![MplsEntry { */
/*             label: 1, */
/*             exp: 2, */
/*             bottom_of_stack: 3, */
/*             ttl: 4, */
/*         }], */
/*         reply_src_addr: Ipv6Addr::from_str("2001:db8::3").unwrap(), */
/*         reply_icmp_type: 0, */
/*         reply_icmp_code: 0, */
/*         rtt: 314, */
/*     }); */
/*     assert_eq!( */
/*         replies, */
/*         IrisTraceroute::from_internal(&replies).to_internal() */
/*     ) */
/* } */
