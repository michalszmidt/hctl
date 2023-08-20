// use std::collections::HashMap;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

/*
use std::sync::Arc;
use rustls::{ClientConfig, ProtocolVersion, RootCertStore, OwnedTrustAnchor};
*/

// pub fn a() -> HashMap<String, Resolver> {}

pub fn default_resolvers() -> Vec<Resolver> {
    let system = Resolver::from_system_conf().unwrap();
    let cloudflare =
        Resolver::new(ResolverConfig::cloudflare_tls(), ResolverOpts::default()).unwrap();
    let quad9 = Resolver::new(ResolverConfig::quad9_tls(), ResolverOpts::default()).unwrap();

    return vec![system, cloudflare, quad9];
}

pub fn system_resolver() -> Vec<Resolver> {
    let system = Resolver::from_system_conf().unwrap();
    return vec![system];
}

pub fn valid_resolv_domain(domain: &String, resolvers: Vec<Resolver>) -> bool {
    // let resp: Vec<_> = resolvers
    //     .par_iter()
    //     .map(|resolver| resolver.lookup_ip(domain))
    //     .filter_map(|result| result.ok())
    //     .collect();

    // if resp.is_empty() {
    //     return false;
    // }

    // return true;

    //TODO err check timeout

    for resolver in resolvers {
        if resolver.lookup_ip(domain).is_ok() {
            return true;
        }
    }

    return false;
}
