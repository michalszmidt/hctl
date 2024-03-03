#[cfg(test)]
mod tests {
    use {
        crate::{
            io::resolver::{from_config_dot_reslver, from_config_plain_reslver},
            logic::structs::HCTL,
        },
        rayon::prelude::{IntoParallelRefIterator, ParallelIterator},
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
