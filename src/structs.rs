use serde::Deserialize;

#[derive(Deserialize)]
pub struct HCTL {
    pub settings: Settings,
    pub remote_sources: Vec<String>,
    pub whitelist: Vec<String>,
    pub remote_whitelist: Vec<String>,
}

#[derive(Deserialize)]
pub struct Settings {
    pub whitelist_include_subdomains: bool,
}
