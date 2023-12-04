#[cfg(test)]
mod tests {
    // use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

    // use crate::rules::regex_subdomain_from_to;

    // #[test]
    // fn some_specific_domains() {
    //     // url::Url::parse(input)
    //     let pattern_valid_domain = get_regex_valid_domain_permissive();
    //     let pattern_basic = get_regex_extract_basic();

    //     let p = vec![
    //         "hel.lo.world",
    //         "example.com",
    //         "hell323232.dom.ain",
    //         "noturl",
    //         "0.0.0.0",
    //         "127.0.0.1",
    //         "0.0.0.0-domain.com",
    //         "0.0.0.0-0.de",
    //         "0.0.0.0-0.de            # comment",
    //         "0.0.0.0                0.0.0.0-0.de                # comment",
    //         "127.0.0.1                0.0.0.0-0.de                # comment",
    //         " ",
    //         "",
    //         "\t\t\t",
    //     ];

    //     // let mut res: Vec<bool> = Vec::new();

    //     println!();
    //     for i in &p {
    //         let re = pattern_valid_domain.is_match(i);
    //         let proc = pattern_basic.replace_all(i, "");
    //         println!("Is domain: {}\t{}", re, proc.to_string());
    //     }
    //     println!();
    // }

    // #[test]
    // fn pattern_subdomain() {
    //     // url::Url::parse(input)
    //     let pattern_valid_subdomain =
    //         regex_subdomain_from_to(2.to_string(), 3.to_string(), "domain.com".to_string());
    //     // let pattern_valid_subdomain_sub = get_regex_subdomain("sub.example.com".to_string());
    //     let vee = vec![
    //         "domain.com",
    //         "sub1.domain.com",
    //         "sub1.sub2.domain.com",
    //         "sub.sub1.sub2.domain.com",
    //         "sub.sub.sub1.sub2.domain.com",
    //     ];
    //     vee.par_iter()
    //         .filter(|x| !pattern_valid_subdomain.is_match(x))
    //         .for_each(|x| println!("|{}|", x));
    // }

    // use crate::resolver::{inbuilt_resolvers, valid_resolv_domain};

    // use std::collections::LinkedList;

    use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

    use crate::{
        resolver::{from_config_dot_reslver, from_config_plain_reslver},
        structs::HCTL,
    };

    #[test]
    fn resolver_yaml_lookup() {
        let s = "
settings:
  whitelist_include_subdomains: true
    
remote_sources:  
    - 
        url: https://v.firebog.net/hosts/static/w3kbl.txt
        src_type: hosts

whitelist:

remote_whitelist:

resolvers:
  -
    usetls: true
    ips:
      - 91.239.100.100
    port: 853
    resolvname: anycast.uncensoreddns.org
    trust_nx: true
  -
    usetls: false
    ips:
      - 1.1.1.1
    port: 53
    resolvname: ''
    trust_nx: true
";
        let hctl: HCTL = serde_yaml::from_str(s).unwrap();

        // If correct
        hctl.resolvers
            .par_iter()
            .map(|resolver| {
                if resolver.usetls {
                    return from_config_dot_reslver(
                        resolver.ips.as_slice(),
                        resolver.port,
                        resolver.resolvname.to_string(),
                        resolver.trust_nx,
                    );
                }
                return from_config_plain_reslver(
                    resolver.ips.as_slice(),
                    resolver.port,
                    resolver.trust_nx,
                );
            })
            .for_each(|resolver| assert_eq!(resolver.lookup_ip("example.com").is_ok(), true));

        // assert_ne!(resolvers[0], None);
    }
}
