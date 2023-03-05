use warts::Traceroute as WartsTraceroute;

pub struct ScamperTraceWarts {
    pub cycle_id: u32,
    pub monitor_name: String,
    pub traceroute: WartsTraceroute,
}
