use std::net::Ipv4Addr;

#[derive(FromForm, Debug)]
pub struct DiscoveryRequest {
    pub ip: Option<Ipv4Addr>,
    pub port: u16,
    pub requested_by: u32,
    pub looking_for: u32,
    pub public_key: String, // Lets get proper parsing on this value done at some point
}