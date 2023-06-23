use crate::regex::{get_regex_extract_basic, get_regex_valid_domain, get_regex_whitespace};
use indicatif::{ProgressIterator, ProgressStyle};
use itertools::*;
use minreq::{get, Error};
use rayon::prelude::{IntoParallelIterator, ParallelIterator, *};
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{self, ErrorKind::WouldBlock, *};
use std::sync::{Arc, Mutex};
use yaml_rust::*;

pub fn process_parallel_list_to_file(
    list_path: String,
    out_path: String,
    save_rejected: bool,
) -> (usize, usize) {
    let pattern = get_regex_extract_basic();
    // let pattern_whitespace = get_regex_whitespace();
    let pattern_url = get_regex_valid_domain();
    let whitespace_pattern = get_regex_whitespace();

    let fil = file_to_lines(list_path).unwrap();
    let reader = BufReader::new(fil);

    let file2 = file_write(out_path).unwrap();
    let mut writer = LineWriter::new(file2);

    let file3 = file_write("./rejected.txt".to_string()).unwrap();
    let mut writer2 = LineWriter::new(file3);

    let rejected = Arc::new(Mutex::new(BTreeSet::new()));
    let mut entries: usize = 0;

    reader
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.starts_with('#'))
        .filter(|x| !x.eq(""))
        .collect::<BTreeSet<_>>()
        .par_iter()
        .map(|x| pattern.replace_all(x, "").to_string())
        .map(|x| {
            whitespace_pattern
                .replace_all(x.as_str(), "")
                .to_string()
                .to_lowercase()
        })
        .filter(|x| {
            let res = pattern_url.is_match(x);
            if !res{ rejected.lock().unwrap().insert(x.clone()); }
            return  res;
            })
        .collect::<BTreeSet<_>>()
        .iter()
        .progress_with_style(
            ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap())
        .for_each(|x| {
            {
                entries+=1;
                _ = writer.write_all(x.as_bytes());
                _ = writer.write_all(b"\n");
            };
        });

    if save_rejected {
        rejected.lock().unwrap().iter().for_each(|x| {
            {
                _ = writer2.write_all(x.as_bytes());
                _ = writer2.write_all(b"\n");
            };
        });
        _ = writer2.flush();
    } else {
        _ = fs::remove_file("./rejected.txt");
    }

    _ = writer.flush();
    return (entries, rejected.lock().unwrap().len());
}

pub fn process_single_list_seq_file(
    list_path: String,
    out_path: String,
    save_rejected: bool,
) -> (usize, usize) {
    let ultimate_pattern = get_regex_extract_basic();
    // let white_char_pattern = get_regex_whitespace();
    let whitespace_pattern = get_regex_whitespace();
    let url_pattern = get_regex_valid_domain();

    let fil = file_to_lines(list_path).unwrap();
    let reader = BufReader::new(fil);
    let file2 = file_write(out_path).unwrap();
    let mut writer = LineWriter::new(file2);

    let file3 = file_write("./rejected.txt".to_string()).unwrap();
    let mut writer2 = LineWriter::new(file3);

    let mut rejected: BTreeSet<String> = BTreeSet::new();
    let mut entries: usize = 0;

    reader
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.starts_with('#'))
        .map(|x| ultimate_pattern.replace_all(x.as_str(), "").to_string())
        .map(|x| {
            whitespace_pattern
                .replace_all(x.as_str(), "")
                .to_string()
                .to_lowercase()
        })
        .filter(|x| {
            let res = url_pattern.is_match(x);
            if !res{ rejected.insert(x.clone()); }
            return  res;
            })
        .unique()
        .sorted()
        .progress_with_style(
            ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap())
        .for_each(|x| {
            {
                entries+=1;
                _ = writer.write_all(x.as_bytes());
                _ = writer.write_all(b"\n");
            };
        });

    if save_rejected {
        rejected.iter().for_each(|x| {
            {
                _ = writer2.write_all(x.as_bytes());
                _ = writer2.write_all(b"\n");
            };
        });
        _ = writer2.flush();
    } else {
        _ = fs::remove_file("./rejected.txt");
    }

    _ = writer.flush();

    return (entries, rejected.len());
}

pub fn process_single_list_to_set(list_path: &String) -> (BTreeSet<String>, BTreeSet<String>) {
    let ultimate_pattern = get_regex_extract_basic();
    let url_pattern = get_regex_valid_domain();
    let whitespace_pattern = get_regex_whitespace();

    let fil = file_to_lines(list_path.clone()).unwrap();
    let reader = BufReader::new(fil);

    let mut rejected: BTreeSet<String> = BTreeSet::new();

    let clean = reader
        .lines()
        .map(|x| x.unwrap())
        .filter(|x| !x.starts_with('#'))
        .map(|x| ultimate_pattern.replace_all(x.as_str(), "").to_string())
        // .filter(|x| !white_char_pattern.is_match(x.as_str()))
        .map(|x| {
            whitespace_pattern
                .replace_all(x.as_str(), "")
                .to_string()
                .to_lowercase()
        })
        .filter(|x| {
            let res = url_pattern.is_match(x);
            if !res {
                rejected.insert(x.clone());
            }
            return res;
        })
        // .filter(|x| !x.eq(""))
        .collect::<BTreeSet<_>>();

    return (clean, rejected);
}

pub fn process_multiple_lists_to_file(
    list_dir: String,
    out_path: String,
    save_rejected: bool,
) -> (usize, usize) {
    let file2 = file_write(out_path).unwrap();
    let mut writer = LineWriter::new(file2);

    let file3 = file_write("./rejected.txt".to_string()).unwrap();
    let mut writer2 = LineWriter::new(file3);

    // let mut rejected: Vec<BTreeSet<String>> = Vec::new();
    let rejected = Arc::new(Mutex::new(BTreeSet::new()));
    let mut entries: usize = 0;

    fs::read_dir(list_dir.as_str())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .par_iter()
        .map(|x| process_single_list_to_set(x))
        .map(|(x,y)| {
            rejected.lock().unwrap().extend(y);
            return x;
        })
        .collect::<Vec<_>>()
        .par_iter()
        .flatten()
        .collect::<BTreeSet<_>>()
        .iter()
        .progress_with_style(
            ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap())
        .for_each(|x| {
            entries +=1;
            _ = writer.write_all(x.as_bytes());
            _ = writer.write_all(b"\n");
        });

    if save_rejected {
        rejected.lock().unwrap().iter().for_each(|x| {
            {
                _ = writer2.write_all(x.as_bytes());
                _ = writer2.write_all(b"\n");
            };
        });
        _ = writer2.flush();
    } else {
        _ = fs::remove_file("./rejected.txt");
    }

    _ = writer.flush();
    return (entries, rejected.lock().unwrap().len());
}

pub fn file_to_lines(path: String) -> io::Result<File> {
    let file = File::open(path)?;
    return Ok(file);
}

pub fn file_write(path: String) -> io::Result<File> {
    let file = File::create(path)?;
    return Ok(file);
}

pub fn file_yaml_to_settings(
    path: String,
    out_path: String,
    intro: bool,
    save_rejected: bool,
) -> (usize, usize) {
    // let white_char_pattern = get_regex_whitespace();

    let input = read_to_string(file_to_lines(path).unwrap()).unwrap();
    let parsed_as_vec = YamlLoader::load_from_str(input.as_str()).unwrap();
    let parsed = &parsed_as_vec[0];
    let sources = parsed["remote_sources"].as_vec().unwrap();

    let file2 = file_write(out_path).unwrap();
    let mut writer = LineWriter::new(file2);

    let file3 = file_write("./rejected.txt".to_string()).unwrap();
    let mut writer2 = LineWriter::new(file3);

    // let mut rejected: Vec<BTreeSet<String>> = Vec::new();
    let rejected = Arc::new(Mutex::new(BTreeSet::new()));
    let mut entries: usize = 0;

    if intro {
        let sources2: Vec<String> = sources
            .clone()
            .into_iter()
            .map(|x| x.into_string().unwrap())
            .collect();
        _ = writer.write_all(
            "# This hostlist was assembled \n# Using: https://github.com//hctl by Michael Szmidt \n# From other lists:\n"
                .as_bytes(),
        );

        sources2.iter().for_each(|line| {
            _ = writer.write_all("# \t- ".as_bytes());
            _ = writer.write_all(line.as_bytes());
            _ = writer.write_all("\n".as_bytes());
        });
    }

    sources
        .into_par_iter()
        .map(|x| lazy_read(x.as_str().unwrap()))
        .filter_map(|x| x.ok())
        .map(|(x,y)| {
            rejected.lock().unwrap().extend(y);
            return x;
        })
        .collect::<Vec<_>>()
        .into_par_iter()
        .flatten()
        // .filter(|x| !white_char_pattern.is_match(x.as_str()))
        .collect::<BTreeSet<_>>()
        .iter()
        .progress_with_style(
            ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap())
        .for_each(|x| {
            entries+=1;
            _ = writer.write_all(x.as_bytes());
            _ = writer.write_all(b"\n");
        });
    if save_rejected {
        rejected.lock().unwrap().iter().for_each(|x| {
            {
                _ = writer2.write_all(x.as_bytes());
                _ = writer2.write_all(b"\n");
            };
        });
        _ = writer2.flush();
    } else {
        _ = fs::remove_file("./rejected.txt");
    }
    _ = writer.flush();
    return (entries, rejected.lock().unwrap().len());
}

// This is lazy reading from network method using minireq with least dependencies.
fn lazy_read(x: &str) -> core::result::Result<(BTreeSet<String>, BTreeSet<String>), Error> {
    let mut str_buffer: Vec<char> = Vec::new();
    let mut set: BTreeSet<String> = BTreeSet::new();
    let mut contin = false;
    let transform_pattern = get_regex_extract_basic();
    let whitespace_pattern = get_regex_whitespace();
    // let whitspace_pattern = get_regex_whitespace();
    let url_pattern = get_regex_valid_domain();
    let mut rejected: BTreeSet<String> = BTreeSet::new();
    let mut prev_char: char = 'q';

    for byte in get(x).send_lazy()? {
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

            let m = whitespace_pattern
                .replace(
                    transform_pattern
                        .replace_all(glued_str.as_str(), "")
                        .to_string()
                        .as_str(),
                    "",
                )
                .to_string()
                .to_lowercase();

            if url_pattern.is_match(m.as_str()) {
                set.insert(m);
            } else {
                rejected.insert(m);
            }
            contin = false;
            str_buffer.clear();
            continue;
        }
        if (byte_as_char) == '#' || contin {
            contin = true;
            continue;
        } else {
            str_buffer.reserve(len);
            str_buffer.push(byte_as_char);
            prev_char = byte_as_char;
        }
    }

    let result = match set {
        _ if (set.len() == 0) => {
            println!("Address failed: {x}");
            return Err(Error::AddressNotFound);
        }
        vaild_set => Ok((vaild_set, rejected)),
    };

    return result;
}
