use clap::{Arg, ArgAction, Command};
use indicatif::ProgressStyle;

// COMMANDS
pub fn get_command_domain() -> Command {
    let command_process: Command = Command::new("domain")
        .short_flag('D')
        .long_flag("domain")
        .about("Manipulate on domains");
    return command_process;
}

// ARGS
pub fn get_args_domain() -> Vec<Arg> {
    let arg_path: Arg = Arg::new("path")
        .help("Path to file to be read [path without quotes]")
        .short('p')
        .long("path")
        .action(ArgAction::Set);

    let arg_out: Arg = Arg::new("out")
        .help("Path to the out file [stdout/path without quotes]")
        .short('o')
        .long("out")
        .action(ArgAction::Set);

    let arg_optimize: Arg = Arg::new("optimize")
        .help("Optimize for memory or speed, default: memory [memory/speed]")
        .short('z')
        .long("optimize")
        .action(ArgAction::Set);

    let arg_mode: Arg = Arg::new("mode")
        .help("Process single or multiple lists [single/folder/config/url]")
        .short('m')
        .long("mode")
        .action(ArgAction::Set)
        .requires("config");

    let arg_config: Arg = Arg::new("config")
        .help("Path to config [without qoutes]")
        .short('c')
        .long("config")
        .action(ArgAction::Set)
        .conflicts_with("path");

    let arg_intro: Arg = Arg::new("intro")
        .help("Whether append intro \"Assembled From:...\" to out file from config [true/false]")
        .short('i')
        .long("intro")
        .action(ArgAction::Set);

    let arg_rejected: Arg = Arg::new("rejected")
        .help("Whether save rejected to ./rejected.txt [true/false]")
        .short('r')
        .long("rejected")
        .action(ArgAction::Set);

    let arg_format: Arg = Arg::new("format")
        .help("Type of out format [dnsmasq/loopback/empty/linewise/bind/snort/unbound/machosts/hostperm1/junkbuster/littlesnitch/pdnsd]")
        .short('f')
        .long("format")
        .action(ArgAction::Set);

    let arg_dns: Arg = Arg::new("dns")
        .help("Validate your processed records issuing dns query in predefined servers, config mode allows custom ones [yes/no]")
        .short('d')
        .long("dns")
        .action(ArgAction::Set);
        
    let arg_pattern: Arg = Arg::new("pattern")
        .help("Input format of list [hosts/dnsmasq]")
        .short('a')
        .long("pattern")
        .action(ArgAction::Set);

    let arg_validate: Arg = Arg::new("validate")
        .help(
            "(Non-intrusive): Verbose validate your records in single file issuing dns-over-tls query in hardcoded servers [yes/no]",
        )
        .short('t')
        .long("validate")
        .requires("path")
        .conflicts_with("mode")
        .conflicts_with("optimize")
        .action(ArgAction::Set);

    return vec![
        arg_path,
        arg_out,
        arg_optimize,
        arg_config,
        arg_mode,
        arg_intro,
        arg_rejected,
        arg_format,
        arg_validate,
        arg_dns,
        arg_pattern, 
    ];
}

// PROGRESSBAR

pub fn progressbar_my_default_style() -> ProgressStyle {
    return ProgressStyle::with_template(
        "[{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
    )
    .unwrap();
}
