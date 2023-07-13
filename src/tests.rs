#[cfg(test)]
mod tests {
    use crate::rules::{get_regex_extract_basic, get_regex_valid_domain_permissive};

    #[test]
    fn some_specific_domains() {
        // url::Url::parse(input)
        let pattern_valid_domain = get_regex_valid_domain_permissive();
        let pattern_basic = get_regex_extract_basic();

        let p = vec![
            "hel.lo.world",
            "example.com",
            "hell323232.dom.ain",
            "noturl",
            "0.0.0.0",
            "127.0.0.1",
            "0.0.0.0-domain.com",
            "0.0.0.0-0.de",
            "0.0.0.0-0.de            # comment",
            "0.0.0.0                0.0.0.0-0.de                # comment",
            "127.0.0.1                0.0.0.0-0.de                # comment",
            " ",
            "",
            "\t\t\t",
        ];

        // let mut res: Vec<bool> = Vec::new();

        println!();
        for i in &p {
            let re = pattern_valid_domain.is_match(i);
            let proc = pattern_basic.replace_all(i, "");
            println!("Is domain: {}\t{}", re, proc.to_string());
        }
        println!();
    }
}
