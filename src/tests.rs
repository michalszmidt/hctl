#[cfg(test)]
mod tests {
    use crate::regex::get_regex_valid_domain;

    #[test]
    fn test_url_valid() {
        // url::Url::parse(input)
        let domain_check = get_regex_valid_domain();

        let p = vec![
            "hel.lo.world",
            "example.com",
            "hell323232.dom.ain",
            "noturl",
            "0.0.0.0",
            "127.0.0.1",
            " ",
            "",
            "\t\t\t",
        ];

        // let mut res: Vec<bool> = Vec::new();

        println!();
        for i in &p {
            let re = domain_check.is_match(i);
            println!("{}: {}", re, i);
        }
        println!();
    }
}
