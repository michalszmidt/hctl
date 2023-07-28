use crate::{
    commands::progressbar_my_default_style,
    rules::{
        iterator_map_whitespce, regex_extract_basic, regex_valid_domain_permissive,
        regex_whitespace,
    },
    savers::{self, file_write, io_writer_out, return_saver},
};
use indicatif::ProgressIterator;
use itertools::*;
use minreq::{get, Error};
use rayon::prelude::*;
use std::{
    collections::BTreeSet,
    fs::{read_dir, remove_file, File},
    io::{self, ErrorKind::WouldBlock, *},
    sync::{Arc, Mutex},
    usize,
};
use yaml_rust::*;

/// This function reads file into memory then enables parallel processing
pub fn process_parallel_list_to_file(
    list_path: String,
    out_path: String,
    save_rejected: bool,
    format: String,
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

    read_dir(list_dir.as_str())
        .unwrap()
        .filter_map(|result| result.ok())
        .map(|dir| dir.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .par_iter()
        .map(|line| process_single_list_to_set(line))
        .map(|(set_cleared, set_rejected)| extend_rejected_from_result(set_cleared, set_rejected))
        .collect::<Vec<_>>()
        .par_iter()
        .flatten()
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
) -> (usize, usize) {
    let settings_as_str = read_to_string(file_to_lines(path).unwrap()).unwrap();
    let parsed_settings_yaml = YamlLoader::load_from_str(settings_as_str.as_str()).unwrap();
    let parsed_settings_yaml_first = &parsed_settings_yaml[0];

    let remote_sources = parsed_settings_yaml_first["remote_sources"]
        .as_vec()
        .unwrap();

    let remote_whitelist = parsed_settings_yaml_first["remote_whitelist"]
        .as_vec()
        .unwrap();

    let yaml_whitelist = parsed_settings_yaml_first["whitelist"].as_vec().unwrap();

    let mut writer_out = io_writer_out(out_path);

    let file_rejected = file_write("./rejected.txt".to_string()).unwrap();
    let mut writer_rejected = BufWriter::new(file_rejected);

    let arc_mux_set_rejected = Arc::new(Mutex::new(BTreeSet::new()));
    let mut count_entries: usize = 0;

    let saver_func = return_saver(format.clone());
    let saver_rejected_func = return_saver("linewise".to_string());

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

    // Processing

    if use_intro {
        let sources_cloned: Vec<String> = remote_sources
            .clone()
            .into_iter()
            .map(|yaml| yaml.into_string().unwrap())
            .collect();
        _ = writer_out.write_all("# This hostlist was assembled from other lists:\n".as_bytes());

        sources_cloned.iter().for_each(|line| {
            _ = writer_out.write_all("# \t- ".as_bytes());
            _ = writer_out.write_all(line.as_bytes());
            _ = writer_out.write_all("\n".as_bytes());
        });
    }

    match format.as_str() {
        "empty" | "loopback" => _ = writer_out.write_all(savers::HOSTLIST_SCHEME.as_bytes()),
        _ => _ = writer_out.write_all(b"\n"),
    }

    let mut set_whitelist: BTreeSet<String> = yaml_whitelist
        .into_par_iter()
        .map(|yml| yml.as_str().unwrap().to_string())
        .collect::<BTreeSet<_>>();

    set_whitelist.extend(
        remote_whitelist
            .into_par_iter()
            .map(|yaml| lazy_read(yaml.as_str().unwrap()))
            .filter_map(|result| result.ok())
            .map(|(set_cleaned, _)| {
                return set_cleaned;
            })
            .collect::<Vec<_>>()
            .into_par_iter()
            .flatten()
            .collect::<BTreeSet<_>>(),
    );

    remote_sources
        .into_par_iter()
        .map(|yaml| lazy_read(yaml.as_str().unwrap()))
        .filter_map(|result| result.ok())
        .map(|(set_cleaned, set_rejected)| extend_rejected_from_result(set_cleaned, set_rejected))
        .collect::<Vec<_>>()
        .into_par_iter()
        .flatten()
        .collect::<BTreeSet<_>>()
        .difference(&set_whitelist)
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

// This is lazy reading from network method using minireq with least dependencies.
fn lazy_read(url: &str) -> core::result::Result<(BTreeSet<String>, BTreeSet<String>), Error> {
    let mut str_buffer: Vec<char> = Vec::new();
    let mut set_out: BTreeSet<String> = BTreeSet::new();
    let mut do_continue = false;

    let pattern_basic = regex_extract_basic();
    let pattern_whitespace = regex_whitespace();
    let pattern_valid_domain = regex_valid_domain_permissive();

    let mut set_rejected: BTreeSet<String> = BTreeSet::new();
    let mut prev_char: char = '\n';

    for byte in get(url).send_lazy()? {
        let (byte, len) = match byte {
            Ok((byte, len)) => (byte, len),
            Err(Error::IoError(err)) if err.kind() == WouldBlock => continue,
            Err(err) => return Err(err),
        };

        let byte_as_char = byte as char;

        if (byte_as_char) == '\n' {
            if prev_char == '\r' {
                _ = str_buffer.remove(str_buffer.len() - 1);
                prev_char = str_buffer.get(str_buffer.len() - 1).unwrap().clone();
            }
            let glued_str: String = str_buffer.iter().collect();

            let word_after_whitespace = pattern_whitespace
                .replace(
                    pattern_basic
                        .replace_all(glued_str.as_str(), "")
                        .to_string()
                        .as_str(),
                    "",
                )
                .to_string()
                .to_lowercase();

            if pattern_valid_domain.is_match(word_after_whitespace.as_str()) {
                set_out.insert(word_after_whitespace);
            } else {
                let mut x: String = word_after_whitespace.clone();
                if !pattern_whitespace.is_match(x.as_str()) {
                    x.push_str("\t# source: ");
                    x.push_str(url);
                    set_rejected.insert(x);
                }
            }
            do_continue = false;
            str_buffer.clear();
            continue;
        }
        if (byte_as_char) == '#' || do_continue {
            do_continue = true;
            continue;
        } else {
            str_buffer.reserve(len);
            str_buffer.push(byte_as_char);
            prev_char = byte_as_char;
        }
    }

    let result = match set_out {
        _ if (set_out.len() == 0) => {
            println!("Address failed: {url}");
            return Err(Error::AddressNotFound);
        }
        set_out => Ok((set_out, set_rejected)),
    };

    return result;
}

// pub fn save_stream(data_stream: Iter<String>, out_path: String) -> usize {
//     let file_out = file_write(out_path).unwrap();
//     let mut writer_out = LineWriter::new(file_out);
//     let mut count_entries: usize = 0;

//     data_stream.progress_with_style(
//             progressbar_my_default_style()
//         .for_each(|word| {
//             count_entries+=1;
//             _ = writer_out.write_all(word.as_bytes());
//             _ = writer_out.write_all(b"\n");
//         });

//     return count_entries;
// }
