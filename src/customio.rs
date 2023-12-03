use crate::rules::{ regex_valid_domain_permissive, regex_whitespace,  regex_choose_pattern};
use minreq::{get, Error};
use std::collections::BTreeSet;
use std::io::ErrorKind::WouldBlock;

// This is lazy reading from network method using minireq with least dependencies.
pub fn lazy_read(url: &str, pattern: &String) -> core::result::Result<(BTreeSet<String>, BTreeSet<String>), Error> {
    let mut str_buffer: Vec<char> = Vec::new();
    let mut set_out: BTreeSet<String> = BTreeSet::new();
    let mut do_continue = false;

let pattern_basic = regex_choose_pattern(pattern);
    
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
                
                // println!("{}", word_after_whitespace);

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
