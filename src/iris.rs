//! Iris newline-delimited JSON format.
//!
//! The following ClickHouse query can be used to generate a file conforming to this format:
//! ```sql
//! SELECT
//!     'unknown' AS measurement_uuid, -- replace with measurement UUID
//!     'unknown' AS agent_uuid, -- replace with agent UUID
//!     formatDateTime(min(capture_timestamp), '%Y-%m-%dT%H:%M:%SZ') AS traceroute_start,
//!     probe_protocol,
//!     probe_src_addr,
//!     probe_dst_addr,
//!     probe_src_port,
//!     probe_dst_port,
//!     groupArray((
//!         formatDateTime(capture_timestamp, '%Y-%m-%dT%H:%M:%SZ'),
//!         probe_ttl,
//!         quoted_ttl,
//!         reply_icmp_type,
//!         reply_icmp_code,
//!         reply_ttl,
//!         reply_size,
//!         reply_mpls_labels,
//!         reply_src_addr,
//!         rtt
//!     )) AS replies
//! FROM ... -- replace with results table name
//! WHERE NOT destination_host_reply
//!       AND NOT destination_prefix_reply
//!       AND NOT private_probe_dst_prefix
//!       AND NOT private_reply_src_addr
//!       AND time_exceeded_reply
//!       AND valid_probe_protocol
//! GROUP BY (
//!     probe_protocol,
//!     probe_src_addr,
//!     probe_dst_prefix,
//!     probe_dst_addr,
//!     probe_src_port,
//!     probe_dst_port,
//! )
//! FORMAT JSONEachRow
//! SETTINGS optimize_aggregation_in_order = 1
//! ```
mod from_internal;
mod models;
mod reader;
mod to_internal;
mod writer;

pub use models::*;
pub use reader::*;
pub use writer::*;
