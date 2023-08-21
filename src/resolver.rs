use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

// use std::collections::HashMap;
use trust_dns_resolver::config::*;
use trust_dns_resolver::proto::rr::RecordType;
use trust_dns_resolver::Resolver;

/*
use std::sync::Arc;
use rustls::{ClientConfig, ProtocolVersion, RootCertStore, OwnedTrustAnchor};
*/

// pub fn a() -> HashMap<String, Resolver> {}

// struct CustomNameServerConfig(NameServerConfig);
// impl NameServerConfig for CustomNameServerConfig {}

pub const UNCENSORED_MULTI_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(91, 239, 100, 100)),
    IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x67c, 0x28a4, 0x0, 0x0, 0x0, 0x0, 0x0,
    )),
];

pub fn many_resolvers() -> Vec<Resolver> {
    // let system = Resolver::from_system_conf().unwrap();
    let cloudflare =
        Resolver::new(ResolverConfig::cloudflare_tls(), ResolverOpts::default()).unwrap();
    let quad9 = Resolver::new(ResolverConfig::quad9_tls(), ResolverOpts::default()).unwrap();

    // let uncsrc = NameServerConfigGroup::from_ips_tls(
    //     UNCENSORED_MULTI_IPS,
    //     853,
    //     "anycast.uncensoreddns.org".to_string(),
    //     true,
    // );

    let nmc = NameServerConfig::new(SocketAddr::new(UNCENSORED_MULTI_IPS[0], 853), Protocol::Tls);
    nmc.tls_dns_name = Some("anycast.uncensoreddns.org".to_string());
    // nmc.bind_addr = Some();

    // let uncs = ResolverConfig {
    //     domain: None,
    //     search: vec![],
    //     name_servers: uncsrc,
    // };

    // let uncs = ResolverConfig::new();
    // uncs.name_servers() = &uncsrc;

    // let conf = NameServerConfig::new(SocketAddr::new(UNCENSORED_MULTI_IPS[0], 853));
    // uncs.add_name_server(uncsrc);

    // {
    // domain: None,
    // search: vec![],

    // };
    return vec![cloudflare, quad9];
}
pub fn inbuilt_resolvers() -> Vec<Resolver> {
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

pub fn valid_resolv_domain(domain: &String, resolvers: Vec<Resolver>) -> (bool, usize) {
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

    let mut i = 0;
    for resolver in resolvers {
        // if resolver.lookup(domain, RecordType::ANY).is_ok() {
        //     return (true, i);
        // }

        let resp = resolver.lookup(domain, RecordType::CNAME).unwrap();
        if resp.records().len().clone() > 0 {
            resp.records().iter().for_each(|x| println!("{}\n", x));
            return (true, i);
        }

        i += 1;
    }
    return (false, i + 1);
}
