use crate::{
    commands::progressbar_my_default_style,
    customio::lazy_read,
    resolver::{
        from_config_dot_reslver, from_config_plain_reslver, many_tls_resolvers_tls,
        valid_resolv_domain,
    },
    rules::{
        iterator_map_whitespce, regex_extract_basic, regex_subdomain_all,
        regex_valid_domain_permissive, regex_whitespace,
    },
    savers::{self, file_write, io_writer_out, return_saver},
    structs::HCTL,
};
use chrono::offset::Utc;
use indicatif::ProgressIterator;
use itertools::*;
use rayon::prelude::*;
use regex::Regex;
use std::{
    collections::{BTreeSet, LinkedList},
    fs::{read_dir, remove_file, File},
    io::{self, *},
    sync::{Arc, Mutex},
    usize,
};

/// This function reads file into memory then enables parallel processing
pub fn process_parallel_list_to_file(
    list_path: String,
    out_path: String,
    save_rejected: bool,
    format: String,
    dns: bool,
) -> (usize, usize) {
    let pattern_basic = regex_extract_basic();
    let pattern_valid_domain = regex_valid_domain_permissive();
    let pattern_whitespace = regex_whitespace();

    let file_opened = file_to_lines(list_path).unwrap();
    let reader = BufReader::new(file_opened);

    let mut writer_out = io_writer_out(out_path);

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
        if dns {
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
        .progress_with_style(progressbar_my_default_style())
        .for_each(|word| save_out_entry(word));

    _ = writer_out.flush();

    if save_rejected {
        save_rejected_all();
    } else {
        drop(writer_rejected);
        _ = remove_file("./rejected.txt");
    }

    return (count_entries, arc_mux_set_rejected.lock().unwrap().len());
}

pub fn process_single_list_seq_file(
    list_path: String,
    out_path: String,
    save_rejected: bool,
    format: String,
) -> (usize, usize) {
    // Declaration
    let pattern_basic = regex_extract_basic();
    let pattern_whitespace = regex_whitespace();
    let pattern_valid_domain = regex_valid_domain_permissive();

    let file_opened = file_to_lines(list_path).unwrap();
    let reader = BufReader::new(file_opened);

    let mut writer_out = io_writer_out(out_path);

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
        .progress_with_style(progressbar_my_default_style())
        .for_each(|word| save_out_entry(word));

    _ = writer_out.flush();

    if save_rejected {
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

pub fn process_single_list_to_set(list_path: &String) -> (BTreeSet<String>, BTreeSet<String>) {
    let pattern_basic = regex_extract_basic();
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

pub fn process_multiple_lists_to_file(
    list_dir: String,
    out_path: String,
    save_rejected: bool,
    format: String,
    dns: bool,
) -> (usize, usize) {
    let mut writer_out = io_writer_out(out_path);
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

    // CLOSURES
    let extend_rejected_from_result = |set_cleared, set_rejected| {
        arc_mux_set_rejected.lock().unwrap().extend(set_rejected);
        return set_cleared;
    };

    let mut flush_rejected = || {
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
        if dns {
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

    read_dir(list_dir.as_str())
        .unwrap()
        .filter_map(|result| result.ok())
        .map(|dir| dir.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .par_iter()
        .map(|line| process_single_list_to_set(line))
        .map(|(set_cleared, set_rejected)| extend_rejected_from_result(set_cleared, set_rejected))
        .flatten()
        .filter(|x| validate_dns(x))
        .collect::<BTreeSet<_>>()
        .iter()
        .progress_with_style(progressbar_my_default_style())
        .for_each(|word| {
            count_entries += 1;
            _ = writer_out.write_all(saver_func(word).as_bytes());
        });
    _ = writer_out.flush();

    if save_rejected {
        flush_rejected();
    } else {
        drop(writer_rejected);
        _ = remove_file("./rejected.txt");
    }

    return (count_entries, arc_mux_set_rejected.lock().unwrap().len());
}

pub fn file_to_lines(path: String) -> io::Result<File> {
    let file = File::open(path)?;
    return Ok(file);
}

pub fn config_process_lists(
    path: String,
    out_path: String,
    use_intro: bool,
    save_rejected: bool,
    format: String,
    dns: bool,
) -> (usize, usize) {
    let hctl_yaml_exact: Option<HCTL> = match serde_yaml::from_reader(file_to_lines(path).unwrap())
    {
        Ok(x) => x,
        Err(e) => {
            println!("{}", e);
            None
        }
    };

    let hctl_yaml = &hctl_yaml_exact;

    let resolvers: LinkedList<_> = hctl_yaml
        .clone()
        .unwrap()
        .resolvers
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
        .clone()
        .collect::<_>();

    let mut writer_out = io_writer_out(out_path);
    let file_rejected = file_write("./rejected.txt".to_string()).unwrap();
    let mut writer_rejected = BufWriter::new(file_rejected);
    let arc_mux_set_rejected = Arc::new(Mutex::new(BTreeSet::new()));
    let mut count_entries: usize = 0;
    let saver_func = return_saver(format.clone());
    let saver_rejected_func = return_saver("linewise".to_string());
    let mut set_whitelist: BTreeSet<String> = hctl_yaml
        .clone()
        .unwrap()
        .whitelist
        .into_par_iter()
        .collect::<BTreeSet<_>>();

    set_whitelist.extend(
        set_whitelist
            .clone()
            .into_par_iter()
            .map(|s| lazy_read(s.as_str()))
            .filter_map(|result| result.ok())
            .map(|(set_cleaned, _)| {
                return set_cleaned;
            })
            .collect::<Vec<_>>()
            .into_par_iter()
            .flatten()
            .collect::<BTreeSet<_>>(),
    );

    let subdomains_regex: Vec<Regex> = match hctl_yaml
        .clone()
        .unwrap()
        .settings
        .whitelist_include_subdomains
    {
        true => set_whitelist
            .iter()
            .map(|x| regex_subdomain_all(x))
            .collect(),
        false => Vec::new(),
    };

    // CLOSURES
    let mut flush_rejected = || {
        arc_mux_set_rejected
            .lock()
            .unwrap()
            .iter()
            .for_each(|word| {
                _ = writer_rejected.write_all(saver_rejected_func(&word).as_bytes());
            });
        _ = writer_rejected.flush();
    };

    let extend_rejected_from_result = |set_cleared, set_rejected| {
        arc_mux_set_rejected.lock().unwrap().extend(set_rejected);
        return set_cleared;
    };

    let subdomains = |domain| {
        if subdomains_regex.len() > 0 {
            return !subdomains_regex
                .iter()
                .map(|x| x.is_match(domain))
                .find_or_first(|x| x == &true)
                .unwrap();
        }
        return true;
    };

    let validate_dns = |word: &String| {
        if dns {
            let (isok, resolvernum) = valid_resolv_domain(word, &resolvers);
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

    // Processing

    if use_intro {
        let sources_cloned: Vec<String> = hctl_yaml
            .clone()
            .unwrap()
            .remote_sources
            .clone()
            .into_iter()
            .map(|x| x.url)
            .collect();
        _ = writer_out.write_all("# This hostlist was assembled at: ".as_bytes());
        _ = writer_out.write_all(Utc::now().to_string().as_bytes());
        _ = writer_out.write_all("\n# From other lists:\n".as_bytes());

        sources_cloned.iter().for_each(|line| {
            _ = writer_out.write_all("# \t- ".as_bytes());
            _ = writer_out.write_all(line.as_bytes());
            _ = writer_out.write_all("\n".as_bytes());
        });
    }

    match format.as_str() {
        "empty" | "loopback" => _ = writer_out.write_all(savers::HOSTLIST_SCHEME.as_bytes()),
        "unbound" => _ = writer_out.write_all(savers::UNBOUND_PRE.as_bytes()),
        _ => _ = writer_out.write_all(b"\n"),
    }

    hctl_yaml
        .clone()
        .unwrap()
        .remote_sources
        .into_par_iter()
        .map(|s| lazy_read(s.as_str()))
        .filter_map(|result| result.ok())
        .map(|(set_cleaned, set_rejected)| extend_rejected_from_result(set_cleaned, set_rejected))
        .flatten()
        .collect::<BTreeSet<_>>()
        .par_iter()
        .filter(|x| subdomains(x))
        .filter(|word| validate_dns(word))
        .collect::<BTreeSet<_>>()
        .iter()
        .progress_with_style(progressbar_my_default_style())
        .for_each(|word| {
            count_entries += 1;
            _ = writer_out.write_all(saver_func(word).as_bytes());
        });

    _ = writer_out.flush();

    if save_rejected {
        flush_rejected();
    } else {
        drop(writer_rejected);
        _ = remove_file("./rejected.txt");
    }
    return (count_entries, arc_mux_set_rejected.lock().unwrap().len());
}

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
