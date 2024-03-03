use {
    crate::{
        io::{
            customio::{file_to_lines, get_from_url, lazy_read},
            resolver::{from_config_dot_reslver, from_config_plain_reslver, valid_resolv_domain},
        },
        logic::{
            rules::{regex_starts_with_http, regex_subdomain_all},
            savers::{self, file_write, io_writer_out, return_saver},
            structs::HCTL,
        },
    },
    chrono::Utc,
    itertools::*,
    rayon::prelude::*,
    regex::Regex,
    std::{
        collections::{BTreeSet, LinkedList},
        fs::remove_file,
        io::{BufWriter, Write as _},
        sync::{Arc, Mutex},
    },
};

pub fn config_process_lists(
    path: &String,
    out_path: &String,
    use_intro: &bool,
    save_rejected: &bool,
    format: &String,
    dns: &bool,
) -> (usize, usize) {
    let is_config_url_regex = regex_starts_with_http();
    let is_config_url = is_config_url_regex.is_match(path.clone().as_str());

    let hctl_yaml_exact: Option<HCTL> = match is_config_url {
        is_url if is_url => match serde_yaml::from_str(get_from_url(path).unwrap().as_str()) {
            Ok(x) => x,
            Err(e) => {
                println!("{}", e);
                None
            }
        },
        is_url if !is_url => match serde_yaml::from_reader(file_to_lines(path.clone()).unwrap()) {
            Ok(x) => x,
            Err(e) => {
                println!("{}", e);
                None
            }
        },
        _ => None,
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

    let mut writer_out = io_writer_out(out_path.clone());
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
            .map(|s| lazy_read(s.as_str(), &"hosts".to_string()))
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
        if dns.clone() {
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

    if use_intro.clone() {
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
        .map(|s| lazy_read(s.url.as_str(), &s.src_type.to_string()))
        .filter_map(|result| result.ok())
        .map(|(set_cleaned, set_rejected)| extend_rejected_from_result(set_cleaned, set_rejected))
        .flatten()
        .collect::<BTreeSet<_>>()
        .par_iter()
        .filter(|x| subdomains(x))
        .filter(|word| validate_dns(word))
        .collect::<BTreeSet<_>>()
        .iter()
        // .progress_with_style(progressbar_my_default_style())
        .for_each(|word| {
            count_entries += 1;
            _ = writer_out.write_all(saver_func(word).as_bytes());
        });

    _ = writer_out.flush();

    if save_rejected.clone() {
        flush_rejected();
    } else {
        drop(writer_rejected);
        _ = remove_file("./rejected.txt");
    }
    return (count_entries, arc_mux_set_rejected.lock().unwrap().len());
}
