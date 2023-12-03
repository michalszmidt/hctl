use regex::Regex;


// REGEXES

// standard
pub fn regex_extract_basic() -> Regex {
    // return Regex::new(r"(\#.*$)|((\d\.\d\.\d\.\d\.\ local|127.0.0.1|localhost.localdomain|255.255.255.255|ip6-localhost|ip6-loopback|fe80::1%lo0|ff00::0|fe00::0|ip6-localnet|ip6-mcastprefix|ff02::1|ff02::2|ip6-allrouters|ff02::3|0.0.0.0|ip6-allhosts|ip6-allnodes|broadcasthost|localhost|::1)(\s|\t))|((\ |\t)\#.*$)").unwrap();
    // return Regex::new(r"^((127.0.0.1|localhost.localdomain|255.255.255.255|fe80::1%lo0|ff00::0|fe00::0|ff02::1|ff02::2|ff02::3|0.0.0.0|::1|fe80::1%lo0)(\ |\t)*)|((\ |\t)*\#.*$)").unwrap();
    // return Regex::new(r"^((127\.0\.0\.1|localhost.localdomain|255\.255\.255\.255|fe80::1%lo0|ff00::0|fe00::0|ff02::1|ff02::2|ff02::3|0\.0\.0\.0|::1|fe80::1%lo0)\s*)|(\s*\#.*$)").unwrap();
    return Regex::new(r"^((127\.0\.0\.1|localhost.localdomain|255\.255\.255\.255|fe80::1%lo0|ff00::0|fe00::0|ff02::1|ff02::2|ff02::3|0\.0\.0\.0|::1|fe80::1%lo0)\s{1,})|(\s*\#.*$)").unwrap();
}

pub fn regex_extract_dnsmasq() -> Regex {
    return  Regex::new(r"^(address=/|server=/)|(/0\.0\.0\.0|/127\.0\.0\.1|/)|(\s*\#.*$)").unwrap();
}
pub fn regex_extract_bind() -> Regex {
    return  Regex::new(r##"^(zone\s{1,}")|("\s{1,}\{\s{1,}type\s{1,}master;\s{1,}notify\s{1,}no;\s{1,}file\s{1,}"null\.zone\.file";\s{1,}};)|(\s*//.*$)"##).unwrap();
}
pub fn regex_extract_hostperm1() -> Regex {
    return  Regex::new(r##"^host\s{1,}image\s{1,}2\s{1,}"##).unwrap();
}




pub fn regex_valid_domain_permissive() -> Regex {
    // return Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9](?:\.[a-zA-Z]{2,})+$").unwrap();
    return Regex::new(
        r"^(?:[a-z0-9_](?:[a-z0-9-_]{0,62}[a-z0-9-_])?\.)+[a-z0-9][a-z0-9-_]{0,62}[a-z0-9]$",
    )
    .unwrap();
}

pub fn regex_whitespace() -> Regex {
    return Regex::new(r"\s+").unwrap();
}

pub fn regex_subdomain_all(domain: &String) -> Regex {
    let spl = domain.replace(".", "\\.");
    let mut pattern_str = r"(([a-z0-9-_]){1,}\.){1,}".to_string();
    pattern_str.push_str(spl.as_str());
    pattern_str.push('|');
    pattern_str.push_str(spl.as_str());
    return Regex::new(pattern_str.as_str()).unwrap();
}

pub fn regex_choose_pattern(pattern: &String) -> Regex{
    return match pattern.as_str() {
        "hosts" =>  regex_extract_basic(),
        "dnsmasq" => regex_extract_dnsmasq(),
        "bind" => regex_extract_bind(),
        "hostperm1" => regex_extract_hostperm1(),
        
        _ => regex_extract_basic(),
    };
}


// pub fn regex_subdomain_from_to(from: String, to: String, domain: String) -> Regex {
//     let spl = domain.replace(".", "\\.");
//     let mut pattern_str = format!("(([a-z0-9-_]){{1,}}\\.){{{},{}}}", from, to);
//     pattern_str.push_str(spl.as_str());
//     pattern_str.push('|');
//     pattern_str.push_str(spl.as_str());
//     // println!("{}", pattern_str.clone());
//     return Regex::new(pattern_str.as_str()).unwrap();
// }

// pub fn get_ip4() -> Regex {
//     return Regex::new(r"^(((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)(\.|$)){4})").unwrap();
// }

// pub fn get_ip6() -> Regex {
//     return Regex::new(r"^(([0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,7}:|([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}|([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}|([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}|([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){1,6})|:((:[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(:[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(ffff(:0{1,4}){0,1}:){0,1}((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])|([0-9a-fA-F]{1,4}:){1,4}:((25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(25[0-5]|(2[0-4]|1{0,1}[0-9]){0,1}[0-9]))$").unwrap();
// }

// ITERATOR FUNCTIONS
pub fn iterator_map_whitespce(compiled_whitespace: &Regex, word: String) -> String {
    return compiled_whitespace
        .replace_all(word.as_str(), "")
        .to_string()
        .to_lowercase();
}
