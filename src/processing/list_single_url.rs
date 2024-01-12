use rayon::prelude::*;
use std::{
    collections::BTreeSet,
    io::{BufWriter, Write},
    sync::{Arc, Mutex}, fs::remove_file,
};

use crate::{
    io::{
        customio::get_from_url,
        resolver::{many_tls_resolvers_tls, valid_resolv_domain},
    },
    logic::{
        rules::{
            iterator_map_whitespce, regex_choose_pattern, regex_valid_domain_permissive,
            regex_whitespace,
        },
        savers::{self, file_write, io_writer_out, return_saver},
    },
};

pub fn config_process_url(
    url: &String,
    out_path: &String,
    save_rejected: &bool,
    format: &String,
    dns: &bool,
    pattern: &String,
) -> (usize, usize) {
    let pattern_basic = regex_choose_pattern(pattern);

    let body = match get_from_url(&url) {
        Ok(body) => body,
        Err(err) => panic!("Address failed {} with err {}", url, err),
    };

    let pattern_valid_domain = regex_valid_domain_permissive();
    let pattern_whitespace = regex_whitespace();

    let mut writer_out = io_writer_out(out_path.clone());

    let file_rejected = file_write("./rejected.txt".to_string()).unwrap();
    let mut writer_rejected = BufWriter::new(file_rejected);

    let arc_mux_set_rejected = Arc::new(Mutex::new(BTreeSet::new()));
    let mut count_entries: usize = 0;

    let saver_func = return_saver(format.clone());
    let saver_rejected_func = return_saver("linewise".to_string());

    match format.as_str() {
        "empty" | "loopback" => _ = writer_out.write_all(savers::HOSTLIST_SCHEME.as_bytes()),
        "unbound" => _ = writer_out.write_all(savers::UNBOUND_PRE.as_bytes()),
        _ => _ = writer_out.write_all(b"\n"),
    }

    // Closures are workaround for cannot & to mut value
    let invalid_domain = |word: &String| {
        let is_domain = pattern_valid_domain.is_match(word);
        if !is_domain {
            arc_mux_set_rejected.lock().unwrap().insert(word.clone());
        }
        return is_domain;
    };

    let mut save_out_entry = |word| {
        count_entries += 1;
        _ = writer_out.write_all(saver_func(word).as_bytes());
    };

    let mut save_rejected_all = || {
        arc_mux_set_rejected
            .lock()
            .unwrap()
            .iter()
            .for_each(|word| {
                _ = writer_rejected.write_all(saver_rejected_func(word).as_bytes());
            });
        _ = writer_rejected.flush();
    };

    let validate_dns = |word: &String| {
        if dns.clone() {
            let (isok, resolvernum) = valid_resolv_domain(word, &many_tls_resolvers_tls());
            if !isok {
                let mut rejec = word.clone();
                rejec.push_str("\t# Domain reslution failed at resolver nr. ");
                rejec.push_str(resolvernum.to_string().as_str());
                arc_mux_set_rejected.lock().unwrap().insert(rejec);
            }
            return isok;
        }
        return true;
    };

    body.split_terminator('\n')
        .into_iter()
        .filter(|line| !line.starts_with('#'))
        .collect::<BTreeSet<_>>()
        .par_iter()
        .map(|word| pattern_basic.replace_all(word, "").to_string())
        .map(|word| iterator_map_whitespce(&pattern_whitespace, word))
        .filter(|word| invalid_domain(word))
        .filter(|x| validate_dns(x))
        .collect::<BTreeSet<_>>()
        .iter()
        .for_each(|word| save_out_entry(word));

    _ = writer_out.flush();

    if save_rejected.clone() {
        save_rejected_all();
    } else {
        drop(writer_rejected);
        _ = remove_file("./rejected.txt");
    }

    return (count_entries, arc_mux_set_rejected.lock().unwrap().len());
}
