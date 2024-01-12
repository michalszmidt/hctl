use std::{
    fs::File,
    io::{self, Write},
};

pub static HOSTLIST_SCHEME: &str = "
127.0.0.1\tlocalhost
127.0.0.1\tlocalhost.localdomain
127.0.0.1\tlocal
255.255.255.255\tbroadcasthost
::1\tlocalhost
::1\tip6-localhost
::1\tip6-loopback
fe80::1%lo0\tlocalhost
ff00::0\tip6-localnet
ff00::0\tip6-mcastprefix
ff02::1\tip6-allnodes
ff02::2\tip6-allrouters
ff02::3\tip6-allhosts
0.0.0.0\t0.0.0.0

";

pub static UNBOUND_PRE: &str = "
server:

";

pub fn io_writer_out(out_path: String) -> Box<dyn Write> {
    return match out_path.as_str() {
        "stdout" => Box::new(io::stdout()) as Box<dyn Write>,
        _ => Box::new(file_write(out_path).unwrap()) as Box<dyn Write>,
    };
}
pub fn file_write(path: String) -> io::Result<File> {
    let file = File::create(path)?;
    return Ok(file);
}

fn saver_simple_linewise(word: &String) -> String {
    let mut res = word.clone();
    res.push('\n');
    return res;
}

fn saver_dnsmasq(word: &String) -> String {
    let mut res = "address=/".to_string();
    res.push_str(word);
    res.push_str("/0.0.0.0\n");
    return res;
}

fn saver_empty(word: &String) -> String {
    let mut res = "0.0.0.0\t".to_string();
    res.push_str(word);
    res.push('\n');
    return res;
}

fn saver_loopback(word: &String) -> String {
    let mut res = "127.0.0.1\t".to_string();
    res.push_str(word);
    res.push('\n');
    return res;
}

fn saver_bind(word: &String) -> String {
    let mut res = "zone \"".to_string();
    res.push_str(word);
    res.push_str("\" { type master; notify no; file \"null.zone.file\"; };\n");
    return res;
}

fn saver_snort(word: &String) -> String {
    let mut res = "alert tcp  any $HOME_NET any (msg:\"".to_string();
    res.push_str(word);
    res.push_str("\"; classtype:trojan-activity; priority:1;)\n");
    return res;
}

fn saver_unbound(word: &String) -> String {
    let mut res = "\tlocal-zone: \"".to_string();
    res.push_str(word);
    res.push_str("\" redirect\n");
    res.push_str("\tlocal-zone: \"");
    res.push_str(word);
    res.push_str(" A 127.0.0.1\"\n");
    return res;
}

fn saver_hostperm1(word: &String) -> String {
    let mut res = "host\timage\t2\t".to_string();
    res.push_str(word);
    res.push('\n');
    return res;
}

fn saver_machosts(word: &String) -> String {
    let mut res = word.clone();
    res.push_str(".\tA 127.0.0.1\n");
    return res;
}

fn saver_littlesnitch(word: &String) -> String {
    let mut res =
        "action: deny\ndirection: outgoing\nprocess: any\nowner: me\ndestination: ".to_string();
    res.push_str(word);
    res.push_str("\nport: any\nprotocol: any\nhelp: Built with hctl\n\n");
    return res;
}
fn saver_junkbuster(word: &String) -> String {
    let mut res = ".".to_string();
    res.push_str(word);
    res.push_str("/\n");
    return res;
}

fn saver_pdnsd(word: &String) -> String {
    let mut res = "neg {\n  name=".to_string();
    res.push_str(word);
    res.push_str(";\n  types=domain;\n  }\n\n");
    return res;
}

// TODO
// squid - needs word modification
// netgar - needs num of word
// adblock - needs word modification
// opera - needs word modification, needs prefix

pub fn return_saver(type_of_writer: String) -> fn(&String) -> String {
    match type_of_writer.as_str() {
        "loopback" => saver_loopback,
        "empty" => saver_empty,
        "dnsmasq" => saver_dnsmasq,
        "linewise" => saver_simple_linewise,
        "bind" => saver_bind,
        "snort" => saver_snort,
        "unbound" => saver_unbound,
        "machosts" => saver_machosts,
        "hostperm1" => saver_hostperm1,
        "junkbuster" => saver_junkbuster,
        "littlesnitch" => saver_littlesnitch,
        "pdnsd" => saver_pdnsd,
        _ => {
            println!("No such option for format, falling back to linewise");
            return saver_simple_linewise;
        }
    }
}
