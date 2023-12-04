pub mod commands;
pub mod customio;
pub mod processing;
pub mod resolver;
pub mod rules;
pub mod savers;
pub mod structs;
pub mod tests;

use clap::{parser::ValuesRef, Command};
use commands::{get_args_domain, get_command_domain};
use processing::{
    config_process_lists, process_multiple_lists_to_file, process_parallel_list_to_file,
    process_single_list_seq_file, validate_from_file, config_process_url,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    // MAIN
    let hctl = Command::new("hctl")
        .about("Ultimate hostlist tool")
        .version(VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Michał Szmidt")
        .subcommand(get_command_domain().args(get_args_domain()))
        .get_matches();

    // ACTION

    if let Some(("domain", query_matches)) = hctl.subcommand() {
        let mut out = "./out.txt".to_string();
        let mut path = "NAN".to_string();
        let mut mode = "config".to_string();
        let mut config = "./hctl.yaml".to_string();
        let mut optimize = "memory".to_string();
        let mut intro = "yes".to_string();
        let mut rejected = "no".to_string();
        let mut format = "linewise".to_string();
        let mut dns = "no".to_string();
        let mut validate = "no".to_string();
        let mut pattern = "hosts".to_string();

        let mut rejected_len: usize = 0;
        let mut entries_len: usize = 0; 

        if let Some(value_of_out) = query_matches.get_many::<String>("out") {
            out = get_param(value_of_out);
        }
        if let Some(value_of_path) = query_matches.get_many::<String>("path") {
            path = get_param(value_of_path);
        }
        if let Some(value_of_optimize) = query_matches.get_many::<String>("optimize") {
            optimize = get_param(value_of_optimize);
        }
        if let Some(value_of_config) = query_matches.get_many::<String>("config") {
            config = get_param(value_of_config);
        }
        if let Some(value_of_mode) = query_matches.get_many::<String>("mode") {
            mode = get_param(value_of_mode);
        }
        if let Some(value_of_intro) = query_matches.get_many::<String>("intro") {
            intro = get_param(value_of_intro);
        }
        if let Some(value_of_rejected) = query_matches.get_many::<String>("rejected") {
            rejected = get_param(value_of_rejected);
        }
        if let Some(value_of_format) = query_matches.get_many::<String>("format") {
            format = get_param(value_of_format);
        }
        if let Some(value_of_dns) = query_matches.get_many::<String>("dns") {
            dns = get_param(value_of_dns);
        }
        if let Some(value_of_validate) = query_matches.get_many::<String>("validate") {
            validate = get_param(value_of_validate);
        }
        
        if let Some(value_of_pattern) = query_matches.get_many::<String>("pattern") {
            pattern = get_param(value_of_pattern);
        }
        
        
        let intro_b = match intro.as_str() {
            "yes" => true,
            "no" => false,
            _ => return,
        };
        let rejected_b = match rejected.as_str() {
            "yes" => true,
            "no" => false,
            _ => return,
        };

        let dns_b = match dns.as_str() {
            "yes" => true,
            "no" => false,
            _ => return,
        };

        match validate.as_str() {
            "yes" => validate_from_file(path),
            "no" => match mode.as_str() {
                "single" => {
                    if path.eq("NAN") {
                        println!("No source file defined: use -p flag\nNo action made");
                        return;
                    }
                    match optimize.as_str() {
                        "speed" => {
                            (entries_len, rejected_len) = 
                                process_parallel_list_to_file(
                                    &path,
                                    &out,
                                    &rejected_b,
                                    &format,
                                    &dns_b,
                                    &pattern,
                                )
                        }
                        "memory" => {
                            (entries_len, rejected_len) =
                                process_single_list_seq_file(&path, &out, &rejected_b, &format, &pattern)
                        }
                        _ => return,
                    };
                }
                "folder" => {
                    if path.eq("NAN") {
                        println!("No source file defined: use -p flag\nNo action made");
                        return;
                    }
                    (entries_len, rejected_len) = process_multiple_lists_to_file(
                        &path,
                       &out,
                        &rejected_b,
                        &format,
                        &dns_b, 
                        &pattern,
                    );
                }
                "config" => {
                    (entries_len, rejected_len) = config_process_lists(
                        &config,
                        &out,
                        &intro_b,
                        &rejected_b,
                        &format,
                        &dns_b,
                    )
                }
                "url" => {
                    (entries_len, rejected_len) = config_process_url(&path, &out, &rejected_b, &format, &dns_b, &pattern)
                    }
                _ => return,
            },
            _ => return,
        }
        if out.as_str() != "stdout" && validate.as_str() != "yes" {
            println!(
                "Unique records: {}\nRemoved records: {}\n",
                entries_len, rejected_len
            );
        }
    } else {
        unreachable!()
    }
}

fn get_param(valuesref: ValuesRef<String>) -> String {
    let x = valuesref.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
    return x;
}