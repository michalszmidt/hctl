use crate::{
    io::{
        customio::file_to_lines,
        resolver::{many_tls_resolvers_tls, valid_resolv_domain},
    },
    logic::rules::{regex_extract_basic, regex_valid_domain_permissive, regex_whitespace, iterator_map_whitespce},
};
use rayon::prelude::*;
use std::{io::{BufReader, BufRead as _}, collections::BTreeSet};

pub fn validate_from_file(list_path: String) {
    let pattern_basic = regex_extract_basic();
    let pattern_valid_domain = regex_valid_domain_permissive();
    let pattern_whitespace = regex_whitespace();

    let file_opened = file_to_lines(list_path).unwrap();
    let reader = BufReader::new(file_opened);

    // Closures are workaround for cannot & to mut value

    let validate_dns = |word: &String| {
        let (isok, _) = valid_resolv_domain(word, &many_tls_resolvers_tls());
        println!("{isok:#?}: {word:#?}");
    };

    reader
        .lines()
        .map(|res| res.unwrap())
        .filter(|line| !line.starts_with('#'))
        .filter(|line| !line.eq(""))
        .collect::<BTreeSet<_>>()
        .par_iter()
        .map(|word| pattern_basic.replace_all(word, "").to_string())
        .map(|word| iterator_map_whitespce(&pattern_whitespace, word))
        .filter(|word| pattern_valid_domain.is_match(word))
        .for_each(|x| validate_dns(&x));
}
