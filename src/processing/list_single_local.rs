use std::{
    collections::BTreeSet,
    fs::remove_file,
    io::{BufRead as _, BufReader, BufWriter, Write as _},
    sync::{Arc, Mutex},
};

use itertools::*;
use rayon::prelude::*;

use crate::{
    io::{
        customio::file_to_lines,
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

pub fn process_parallel_list_to_file(
    list_path: &String,
    out_path: &String,
    save_rejected: &bool,
    format: &String,
    dns: &bool,
    pattern: &String,
) -> (usize, usize) {
    let pattern_basic = regex_choose_pattern(pattern);

    let pattern_valid_domain = regex_valid_domain_permissive();
    let pattern_whitespace = regex_whitespace();

    let file_opened = file_to_lines(list_path.clone()).unwrap();
    let reader = BufReader::new(file_opened);

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

    reader
        .lines()
        .map(|res| res.unwrap())
        .filter(|line| !line.starts_with('#'))
        .filter(|line| !line.eq(""))
        .collect::<BTreeSet<_>>()
        .par_iter()
        .map(|word| pattern_basic.replace_all(word, "").to_string())
        .map(|word| iterator_map_whitespce(&pattern_whitespace, word))
        .filter(|word| invalid_domain(word))
        .filter(|x| validate_dns(x))
        .collect::<BTreeSet<_>>()
        .iter()
        // .progress_with_style(progressbar_my_default_style())
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

pub fn process_single_list_seq_file(
    list_path: &String,
    out_path: &String,
    save_rejected: &bool,
    format: &String,
    pattern: &String,
) -> (usize, usize) {
    // Declaration
    let pattern_basic = regex_choose_pattern(pattern);

    let pattern_whitespace = regex_whitespace();
    let pattern_valid_domain = regex_valid_domain_permissive();

    let file_opened = file_to_lines(list_path.clone()).unwrap();
    let reader = BufReader::new(file_opened);

    let mut writer_out = io_writer_out(out_path.clone());

    let file_rejected = file_write("./rejected.txt".to_string()).unwrap();
    let mut writer_rejected = BufWriter::new(file_rejected);

    let mut set_rejected: BTreeSet<String> = BTreeSet::new();
    let mut count_entries: usize = 0;

    let saver_func = return_saver(format.clone());
    let saver_rejected_func = return_saver("linewise".to_string());

    match format.as_str() {
        "empty" | "loopback" => _ = writer_out.write_all(savers::HOSTLIST_SCHEME.as_bytes()),
        "unbound" => _ = writer_out.write_all(savers::UNBOUND_PRE.as_bytes()),
        _ => _ = writer_out.write_all(b"\n"),
    }

    // Closures are workaround for cannot reference to mut value
    let mut invalid_domain = |word: &String| {
        let res = pattern_valid_domain.is_match(word);
        if !res {
            set_rejected.insert(word.clone());
        }
        return res;
    };

    let mut save_out_entry = |word| {
        count_entries += 1;
        _ = writer_out.write_all(saver_func(&word).as_bytes());
    };

    // Processing
    reader
        .lines()
        .map(|result| result.unwrap())
        .filter(|line| !line.starts_with('#'))
        .map(|word| pattern_basic.replace_all(word.as_str(), "").to_string())
        .map(|word| iterator_map_whitespce(&pattern_whitespace, word))
        .unique()
        .filter(|word| invalid_domain(word))
        .sorted()
        // .progress_with_style(progressbar_my_default_style())
        .for_each(|word| save_out_entry(word));

    _ = writer_out.flush();

    if save_rejected.clone() {
        set_rejected.iter().for_each(|word| {
            _ = writer_rejected.write_all(saver_rejected_func(word).as_bytes());
        });
        _ = writer_rejected.flush();
    } else {
        drop(writer_rejected);
        _ = remove_file("./rejected.txt");
    }

    return (count_entries, set_rejected.len());
}

pub fn process_single_list_to_set(
    list_path: &String,
    pattern: &String,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let pattern_basic = regex_choose_pattern(pattern);

    let pattern_valid_domain = regex_valid_domain_permissive();
    let pattern_whitespace = regex_whitespace();

    let file_opened = file_to_lines(list_path.clone()).unwrap();
    let reader = BufReader::new(file_opened);

    let mut set_rejected: BTreeSet<String> = BTreeSet::new();

    //CLOSUERS
    let mut invalid_domain = |word: &String| {
        let res = pattern_valid_domain.is_match(word);
        if !res {
            let mut x: String = word.clone();
            if !pattern_whitespace.is_match(x.as_str()) {
                x.push_str("\t# source: ");
                x.push_str(list_path);
                set_rejected.insert(x);
            }
        }
        return res;
    };

    let set_cleaned = reader
        .lines()
        .map(|result| result.unwrap())
        .filter(|line| !line.starts_with('#'))
        .map(|word| pattern_basic.replace_all(word.as_str(), "").to_string())
        .map(|word| iterator_map_whitespce(&pattern_whitespace, word))
        .filter(|word| invalid_domain(word))
        .collect::<BTreeSet<_>>();

    return (set_cleaned, set_rejected);
}
