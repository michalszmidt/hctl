use rand::Rng;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use trust_dns_resolver::config::*;
use trust_dns_resolver::proto::rr::RecordType;
use trust_dns_resolver::Resolver;

pub const UNCENSORED_DNS_MULTI_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(91, 239, 100, 100)),
    IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x67c, 0x28a4, 0x0, 0x0, 0x0, 0x0, 0x0,
    )),
];

pub const DNS_NJALLA_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(95, 215, 19, 53)),
    IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x67c, 0x2354, 0x2, 0x0, 0x0, 0x0, 0x53,
    )),
];

pub const OPENNAMESERVER_ORG_R1_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(217, 160, 70, 42)),
    IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x8d8, 0x1801, 0x86e7, 0x0, 0x0, 0x0, 0x1,
    )),
];

pub const OPENNAMESERVER_ORG_R2_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(178, 254, 22, 166)),
    IpAddr::V6(Ipv6Addr::new(
        0x2a00, 0x6800, 0x3, 0x4bd, 0x0, 0x0, 0x0, 0x1,
    )),
];

pub const OPENNAMESERVER_ORG_R3_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(81, 169, 136, 222)),
    IpAddr::V6(Ipv6Addr::new(
        0x2a01, 0x238, 0x4231, 0x5200, 0x0, 0x0, 0x0, 0x1,
    )),
];

pub const OPENNAMESERVER_ORG_R4_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(185, 181, 61, 24)),
    IpAddr::V6(Ipv6Addr::new(
        0x2a03, 0x94e0, 0x1804, 0x0, 0x0, 0x0, 0x0, 0x1,
    )),
];

pub const DIGITALE_GESELLSCHAFT_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(185, 95, 218, 43)),
    IpAddr::V4(Ipv4Addr::new(185, 95, 218, 42)),
];

pub const DNS_WATCH_R1_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(84, 200, 69, 80)),
    IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x1608, 0x10, 0x25, 0x0, 0x0, 0x1c04, 0xb12f,
    )),
];

pub const DNS_WATCH_R2_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(84, 200, 70, 40)),
    IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x1608, 0x10, 0x25, 0x0, 0x0, 0x9249, 0xd69b,
    )),
];

pub const APPLIED_PRIVACY_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(146, 255, 56, 98)),
    IpAddr::V6(Ipv6Addr::new(
        0x2a02, 0x1b8, 0x10, 0x234, 0x0, 0x0, 0x0, 0x2,
    )),
];

pub const DIGITALSIZE_NET_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(94, 130, 135, 203)),
    IpAddr::V6(Ipv6Addr::new(
        0x2a01, 0x4f8, 0x13b, 0x3407, 0x0, 0x0, 0x0, 0xface,
    )),
];

pub const IBKSTURM_SYNOLOGY_ME_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(213, 196, 191, 96)),
    IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x470, 0x26, 0x7c, 0xb26e, 0xbfff, 0xfe1d, 0xe19b,
    )),
];

pub const SEBY_IO_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(45, 76, 113, 31)),
    IpAddr::V4(Ipv4Addr::new(139, 99, 222, 72)),
];

pub const FFMUC_DE_IPS: &[IpAddr] = &[
    IpAddr::V4(Ipv4Addr::new(5, 1, 66, 255)),
    IpAddr::V4(Ipv4Addr::new(185, 150, 99, 255)),
    IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x678, 0xe68, 0xf000, 0x0, 0x0, 0x0, 0x0,
    )),
    IpAddr::V6(Ipv6Addr::new(
        0x2001, 0x678, 0xed0, 0xf000, 0x0, 0x0, 0x0, 0x0,
    )),
];

pub fn many_resolvers_tls() -> Vec<Resolver> {
    // let system = Resolver::from_system_conf().unwrap();
    let cloudflare =
        Resolver::new(ResolverConfig::cloudflare_tls(), ResolverOpts::default()).unwrap();
    let quad9 = Resolver::new(ResolverConfig::quad9_tls(), ResolverOpts::default()).unwrap();

    let uncensored_dns_tls_cg = NameServerConfigGroup::from_ips_tls(
        UNCENSORED_DNS_MULTI_IPS,
        853,
        "anycast.uncensoreddns.org".to_string(),
        true,
    );
    let uncensored_dns_tls_rc = ResolverConfig::from_parts(None, vec![], uncensored_dns_tls_cg);

    let ffmc_de_tls_cg =
        NameServerConfigGroup::from_ips_tls(FFMUC_DE_IPS, 853, "dot.ffmuc.net".to_string(), true);
    let ffmuc_de_tls_rc = ResolverConfig::from_parts(None, vec![], ffmc_de_tls_cg);

    let dns_watch_r1_tls_cg = NameServerConfigGroup::from_ips_tls(
        DNS_WATCH_R1_IPS,
        853,
        "resolver1.dns.watch".to_string(),
        true,
    );
    let dns_watch_r1_tls_rc = ResolverConfig::from_parts(None, vec![], dns_watch_r1_tls_cg);

    let dns_watch_r2_tls_cg = NameServerConfigGroup::from_ips_tls(
        DNS_WATCH_R2_IPS,
        853,
        "resolver2.dns.watch".to_string(),
        true,
    );
    let dns_watch_r2_tls_rc = ResolverConfig::from_parts(None, vec![], dns_watch_r2_tls_cg);

    let seby_io_tls_cg =
        NameServerConfigGroup::from_ips_tls(SEBY_IO_IPS, 853, "dot.seby.io".to_string(), true);
    let seby_io_tls_rc = ResolverConfig::from_parts(None, vec![], seby_io_tls_cg);

    let applied_privacy_tls_cg = NameServerConfigGroup::from_ips_tls(
        APPLIED_PRIVACY_IPS,
        853,
        "dot1.applied-privacy.net".to_string(),
        true,
    );
    let applied_privacy_tls_rc = ResolverConfig::from_parts(None, vec![], applied_privacy_tls_cg);

    let digitalsize_net_tls_cg = NameServerConfigGroup::from_ips_tls(
        DIGITALSIZE_NET_IPS,
        853,
        "dns.digitalsize.net".to_string(),
        true,
    );
    let digitalsize_net_tls_rc = ResolverConfig::from_parts(None, vec![], digitalsize_net_tls_cg);

    let ibksturm_synology_me_tls_cg = NameServerConfigGroup::from_ips_tls(
        IBKSTURM_SYNOLOGY_ME_IPS,
        853,
        "ibksturm.synology.me".to_string(),
        true,
    );
    let ibksturm_synology_me_tls_rc =
        ResolverConfig::from_parts(None, vec![], ibksturm_synology_me_tls_cg);

    let digitale_gesellschaft_ch_tls_cg = NameServerConfigGroup::from_ips_tls(
        DIGITALE_GESELLSCHAFT_IPS,
        853,
        "digitale.gesellschaft.ch".to_string(),
        true,
    );
    let digitale_gesellschaft_ch_tls_rc =
        ResolverConfig::from_parts(None, vec![], digitale_gesellschaft_ch_tls_cg);

    let njalla_tls_cg =
        NameServerConfigGroup::from_ips_tls(DNS_NJALLA_IPS, 853, "dns.njal.la".to_string(), true);
    let njalla_tls_tls_rc = ResolverConfig::from_parts(None, vec![], njalla_tls_cg);

    let opennameserver_org_r1_cg = NameServerConfigGroup::from_ips_tls(
        OPENNAMESERVER_ORG_R1_IPS,
        853,
        "ns1.opennameserver.org".to_string(),
        true,
    );
    let opennameserver_org_r1_rc =
        ResolverConfig::from_parts(None, vec![], opennameserver_org_r1_cg);

    let opennameserver_org_r2_cg = NameServerConfigGroup::from_ips_tls(
        OPENNAMESERVER_ORG_R2_IPS,
        853,
        "ns2.opennameserver.org".to_string(),
        true,
    );
    let opennameserver_org_r2_rc =
        ResolverConfig::from_parts(None, vec![], opennameserver_org_r2_cg);

    let opennameserver_org_r3_cg = NameServerConfigGroup::from_ips_tls(
        OPENNAMESERVER_ORG_R3_IPS,
        853,
        "ns3.opennameserver.org".to_string(),
        true,
    );
    let opennameserver_org_r3_rc =
        ResolverConfig::from_parts(None, vec![], opennameserver_org_r3_cg);

    let opennameserver_org_r4_cg = NameServerConfigGroup::from_ips_tls(
        OPENNAMESERVER_ORG_R4_IPS,
        853,
        "ns4.opennameserver.org".to_string(),
        true,
    );
    let opennameserver_org_r4_rc =
        ResolverConfig::from_parts(None, vec![], opennameserver_org_r4_cg);

    let uncensored_dns_tls = Resolver::new(uncensored_dns_tls_rc, ResolverOpts::default()).unwrap();
    let ffmuc_de_tls = Resolver::new(ffmuc_de_tls_rc, ResolverOpts::default()).unwrap();
    let dns_watch_r1_tls = Resolver::new(dns_watch_r1_tls_rc, ResolverOpts::default()).unwrap();
    let dns_watch_r2_tls = Resolver::new(dns_watch_r2_tls_rc, ResolverOpts::default()).unwrap();
    let seby_io_tls = Resolver::new(seby_io_tls_rc, ResolverOpts::default()).unwrap();
    let digitalsize_net_tls =
        Resolver::new(digitalsize_net_tls_rc, ResolverOpts::default()).unwrap();
    let applied_privacy_tls =
        Resolver::new(applied_privacy_tls_rc, ResolverOpts::default()).unwrap();
    let ibksturm_synology_me_tls =
        Resolver::new(ibksturm_synology_me_tls_rc, ResolverOpts::default()).unwrap();
    let digitale_gesellschaft_ch =
        Resolver::new(digitale_gesellschaft_ch_tls_rc, ResolverOpts::default()).unwrap();
    let njalla_tls = Resolver::new(njalla_tls_tls_rc, ResolverOpts::default()).unwrap();
    let opennameserver_org_r1_tls =
        Resolver::new(opennameserver_org_r1_rc, ResolverOpts::default()).unwrap();
    let opennameserver_org_r2_tls =
        Resolver::new(opennameserver_org_r2_rc, ResolverOpts::default()).unwrap();
    let opennameserver_org_r3_tls =
        Resolver::new(opennameserver_org_r3_rc, ResolverOpts::default()).unwrap();
    let opennameserver_org_r4_tls =
        Resolver::new(opennameserver_org_r4_rc, ResolverOpts::default()).unwrap();

    return vec![
        uncensored_dns_tls,
        opennameserver_org_r1_tls,
        opennameserver_org_r2_tls,
        opennameserver_org_r3_tls,
        opennameserver_org_r4_tls,
        ffmuc_de_tls,
        dns_watch_r1_tls,
        dns_watch_r2_tls,
        seby_io_tls,
        applied_privacy_tls,
        digitalsize_net_tls,
        ibksturm_synology_me_tls,
        digitale_gesellschaft_ch,
        njalla_tls,
        cloudflare,
        quad9,
    ];
}

pub fn many_resolvers_tls_moved(num: &usize) -> Vec<Resolver> {
    let mut resolvers = many_resolvers_tls();
    let diff = num % resolvers.len().clone();
    if diff == 0 {
        return resolvers;
    }
    resolvers.rotate_left(diff);
    return resolvers;
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

// pub fn custom_resolver()

pub fn valid_resolv_domain(domain: &String, mut resolvers: Vec<Resolver>) -> (bool, usize) {
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
    let len = resolvers.len();

    let mut i = 0;
    let mut rng = rand::thread_rng();

    resolvers.rotate_right(rng.gen_range(1..len - 1));
    // resc.rot

    for resolver in resolvers {
        if resolver.lookup_ip(domain).is_ok() {
            return (true, i);
        } else {
            if resolver.lookup(domain, RecordType::CNAME).is_ok() {
                return (true, i);
            }
        }
        i += 1;
    }
    return (false, i + 1);
}
