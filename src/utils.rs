use serde::de::IntoDeserializer;
use serde::Deserialize;
use std::net::{IpAddr, Ipv6Addr};

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

pub fn ipv6_from_ip(addr: IpAddr) -> Ipv6Addr {
    match addr {
        IpAddr::V4(x) => x.to_ipv6_mapped(),
        IpAddr::V6(x) => x,
    }
}

pub fn protocol_number(s: &str) -> u8 {
    match s {
        "ICMP" => 1,
        "ICMP6" => 58,
        "TCP" => 6,
        "UDP" => 17,
        _ => panic!("Unsupported protocol: {}", s),
    }
}

pub fn protocol_string(n: u8) -> String {
    match n {
        1 => String::from("ICMP"),
        6 => String::from("TCP"),
        17 => String::from("UDP"),
        58 => String::from("ICMP6"),
        _ => panic!("Unsupported protocol: {}", n),
    }
}
