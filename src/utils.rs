//! Functions common to the various traceroute formats.
use std::net::{IpAddr, Ipv6Addr};

use phf::phf_map;
use serde::de::IntoDeserializer;
use serde::Deserialize;

pub const UNSPECIFIED: IpAddr = IpAddr::V6(Ipv6Addr::UNSPECIFIED);

pub static PROTOCOL_TO_STRING: phf::Map<u8, &'static str> = phf_map! {
    1u8 => "ICMP",
    6u8 => "TCP",
    17u8 => "UDP",
    58u8 => "ICMP6"
};

pub static PROTOCOL_FROM_STRING: phf::Map<&'static str, u8> = phf_map! {
    "ICMP" => 1u8,
    "ICMP6" => 58u8,
    "TCP" => 6u8,
    "UDP" => 17u8
};

pub fn default_ipaddr() -> Option<IpAddr> {
    None
}

pub fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<String>::deserialize(de)?;
    let opt = opt.as_deref();
    match opt {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}
