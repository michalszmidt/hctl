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

pub fn return_saver(type_of_writer: String) -> fn(&String) -> String {
    match type_of_writer.as_str() {
        "loopback" => saver_loopback,
        "empty" => saver_empty,
        "dnsmasq" => saver_dnsmasq,
        "linewise" => saver_simple_linewise,
        _ => {
            println!("No such option for format, falling back to linewise");
            return saver_simple_linewise;
        }
    }
}

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
