use std::{
    collections::BTreeSet,
    fs::{read_dir, remove_file},
    io::{BufWriter, Write as _},
    sync::{Arc, Mutex},
};

use rayon::prelude::*;

use crate::{
    io::resolver::{many_tls_resolvers_tls, valid_resolv_domain},
    logic::savers::{self, file_write, io_writer_out, return_saver},
};

use super::list_single_local::process_single_list_to_set;

pub fn process_multiple_lists_to_file(
    list_dir: &String,
    out_path: &String,
    save_rejected: &bool,
    format: &String,
    dns: &bool,
    pattern: &String,
) -> (usize, usize) {
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

    read_dir(list_dir.as_str())
        .unwrap()
        .filter_map(|result| result.ok())
        .map(|dir| dir.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .par_iter()
        .map(|line| process_single_list_to_set(line, pattern))
        .map(|(set_cleared, set_rejected)| extend_rejected_from_result(set_cleared, set_rejected))
        .flatten()
        .filter(|x| validate_dns(x))
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
