use warts::Traceroute as WartsTraceroute;

pub struct WartsTracerouteWithMeta {
    pub cycle_id: u32,
    pub monitor_name: String,
    pub traceroute: WartsTraceroute,
}
