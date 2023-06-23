use crate::processing::{get_regex_ultimate, get_regex_valid_url, get_regex_whitespace};
// use core::result;
use indicatif::{ProgressIterator, ProgressStyle};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, read_to_string, LineWriter, Write};
use yaml_rust::*;

pub fn file_to_lines(path: String) -> io::Result<File> {
    let file = File::open(path)?;
    return Ok(file);
}

pub fn file_write(path: String) -> io::Result<File> {
    let file = File::create(path)?;
    return Ok(file);
}

pub fn file_from_https() {}

pub fn file_yaml_to_settings(path: String, out_path: String) {
    let white_char_pattern = get_regex_whitespace();

    let input = read_to_string(file_to_lines(path).unwrap()).unwrap();
    let parsed_as_vec = YamlLoader::load_from_str(input.as_str()).unwrap();
    let parsed = &parsed_as_vec[0];
    let sources = parsed["remote_sources"].as_vec().unwrap();

    let file2 = file_write(out_path).unwrap();
    let mut writer = LineWriter::new(file2);

    sources
        .into_par_iter()
        .map(|x| lazy_read(x.as_str().unwrap()))
        .filter_map(|x| x.ok())
        .collect::<Vec<_>>()
        .into_par_iter()
        .flatten()
        .filter(|x| !white_char_pattern.is_match(x.as_str()))
        .collect::<BTreeSet<_>>()
        .iter()
        .progress_with_style(
            ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
        )
        .unwrap())
        .for_each(|x| {
            _ = writer.write_all(x.as_bytes());
            _ = writer.write_all(b"\n");
        });

    _ = writer.flush();
}

// This is lazy reading from network method using minireq with least dependencies.
fn lazy_read(x: &str) -> Result<BTreeSet<String>, minreq::Error> {
    let mut str_buffer: Vec<char> = Vec::new();
    let mut set: BTreeSet<String> = BTreeSet::new();
    let mut contin = false;
    let ultimate_pattern = get_regex_ultimate();
    // let whitspace_pattern = get_regex_whitespace();
    let url_pattern = get_regex_valid_url();

    for byte in minreq::get(x).send_lazy()? {
        let (byte, len) = match byte {
            Ok((byte, len)) => (byte, len),
            Err(minreq::Error::IoError(err)) if err.kind() == std::io::ErrorKind::WouldBlock => {
                continue
            }
            Err(err) => return Err(err),
        };

        let byte_as_char = byte as char;

        if (byte_as_char) == '\n' {
            let glued_str: String = str_buffer.iter().collect();

            let m = ultimate_pattern
                .replace_all(glued_str.as_str(), "")
                .to_string();

            if url_pattern.is_match(m.as_str()) {
                set.insert(m);
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
        }
    }

    let result = match set {
        _ if (set.len() == 0) => {
            println!("Address failed: {x}");
            return Err(minreq::Error::AddressNotFound);
        }
        vaild_set => Ok(vaild_set),
    };

    return result;
}

pub fn files_from_https() {}
