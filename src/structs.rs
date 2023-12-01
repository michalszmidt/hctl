use std::net::IpAddr;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct HCTL {
    pub settings: Settings,
    pub remote_sources: Vec<Source>,
    pub whitelist: Vec<String>,
    pub remote_whitelist: Vec<String>,
    pub resolvers: Vec<HCLResolver>,
}

#[derive(Clone, Copy, Deserialize)]
pub struct Settings {
    pub whitelist_include_subdomains: bool,
    // pub reslover_verbose: bool,
}

#[derive(Clone, Deserialize)]
pub struct HCLResolver {
    pub usetls: bool,
    pub ips: Vec<IpAddr>,
    pub port: u16,
    pub resolvname: String,
    pub trust_nx: bool,
}

#[derive(Clone, Deserialize)]
pub struct Source {
    pub url: String,
    pub src_type: String,
}