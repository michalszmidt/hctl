use std::net::IpAddr;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct HCTL {
    pub settings: Settings,
    pub remote_sources: Vec<String>,
    pub whitelist: Vec<String>,
    pub remote_whitelist: Vec<String>,
    pub resolvers: Vec<HCLResolver>,
}

#[derive(Deserialize)]
pub struct Settings {
    pub whitelist_include_subdomains: bool,
}
#[derive(Deserialize)]
pub struct HCLResolver {
    pub ips: Vec<IpAddr>,
    pub port: u16,
    pub resolvname: String,
    pub trust_nx: bool,
}
