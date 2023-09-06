```
    __            __     __
   / /_   _____  / /_   / /
  / __ \ / ___/ / __/  / / 
 / / / // /__  / /_   / /  
/_/ /_/ \___/  \__/  /_/   
```
<div>
  <img alt="Repo Size" src="https://img.shields.io/github/repo-size/michalszmidt/hctl" />
  <img alt="Lines of code" src="https://sloc.xyz/github/michalszmidt/hctl?category=code" />
  <img alt="Last Commit" src="https://img.shields.io/github/last-commit/michalszmidt/hctl" />
  <img alt="Assets Downloads" src="https://img.shields.io/github/downloads/michalszmidt/hctl/total" />
</div>

# About

Hostlists tools cli `hctl` is cli tool written in rust to manage (merge, transform, clear from comments etc.) your hostlists from diffrent formats to selected by you!

- [Wiki](https://github.com/michalszmidt/hctl/wiki)
- [Usage](https://github.com/michalszmidt/hctl/wiki/Usage)

# Prerequisites, dependencies
### `openssl`

- **\*nix systems (linux, BSD, mac)**: should be already there, if not install it from your package repo like `brew install openssl@3`
- **windows**: install from `choco` or `vcpkg` or whatever other way that won't welcome viruses.

Note that you can get rid of openssl dependency if you want, by building from source with modified dependencies, see [building from source](https://github.com/michalszmidt/hctl/wiki/Developer#building-from-source).

# Downloads

[![Packaging status](https://repology.org/badge/tiny-repos/hctl.svg)](https://repology.org/project/hctl/versions)

[![Packaging status](https://repology.org/badge/vertical-allrepos/hctl.svg)](https://repology.org/project/hctl/versions)

## [From release page](https://github.com/michalszmidt/hctl/releases/latest)

<table>
<tr><th>Platform</th><th>Downloads</th><th>Build Status</th></tr>
<tr><td>

| OS |
|----------|
| Linux glibc |
| Linux musl  |
| MacOS X |
| Windows |
| FreeBSD |
| NetBSD |

</td><td>
    
| amd64 | aarch64 |
|-------|---------|
|[app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-linux-glibc-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-linux-glibc-amd64) | [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-linux-glibc-aarch64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-linux-glibc-aarch64) |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-linux-musl-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-linux-musl-amd64) | [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-linux-musl-aarch64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-linux-musl-aarch64) |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-macosx-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-macosx-amd64) | [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-macosx-silicon), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-macosx-silicon) |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-windows-amd64.exe), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-windows-amd64.exe.txt) | [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-windows-aarch64.exe), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-windows-aarch64.exe.txt) |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-freebsd-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-freebsd-amd64) | NSU |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-netbsd-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-netbsd-amd64) | NSU |

</td><td>

| amd64 | aarch64 |
|-------|---------|
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_linux_glibc_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=LinuxGlibcAarch64)](https://cirrus-ci.com/github/michalszmidt/hctl) |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_linux_musl_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=LinuxMuslAarch64)](https://cirrus-ci.com/github/michalszmidt/hctl) |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_macosx_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=MacosxSilicon)](https://cirrus-ci.com/github/michalszmidt/hctl) |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_windows_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_windows_aarch64.yml)](https://github.com/michalszmidt/hctl/actions) |
| [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=FreebsdAmd64)](https://cirrus-ci.com/github/michalszmidt/hctl) | NSU |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_netbsd_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | NSU |

</td></tr> </table>

<!--
| DragonflyBSD |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-dragonflybsd-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-dragonflybsd-amd64) | N/A |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_dragonflybsd_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | N/A |


| OpenBSD |
|  [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-openbsd-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-openbsd-amd64) NSU | NSU |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_openbsd_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | NSU |
-->


- NSU - Not Set Up Yet, will appear in the future

Note that OpenBSD Builds are removed from table as gh Action OpenBSD runner is broken, see [this issue](https://github.com/vmactions/openbsd-vm/issues/12)

# Features

- [x] Merge hostlist
- [x] Remove duplicates
- [x] Remove commented lines (default behaviour)
- [x] Fetch lists directly from network (https)
- [x] YAML config
- [x] Parallel processing
- [x] Progressbar on save
- [x] Rejected lines may be saved with information about source list
- [x] Support for Windows encoded text files (line ending with `\n\r`)
- [x] Output to stdout to be used with unix pipe stream control (partial support)
- [x] Whitelists (enumerated, from external lists file/https)
- [x] Whitelists including subdomains
- [x] Record existance validation from DNS servers (unencrypted and over tls)
- [ ] Input from stdout to be used with unix pipe stream control
- [ ] YAML has full settings (remote sources, ~whitelist~ , ~args from cli~)
- [ ] Subdomains family recognition, save with subdomain order (like adaway does)
- [ ] Fault-tolerance, better error handling
- [ ] Deep rejected analysis
- [ ] Quiet option

Jump [here](https://github.com/michalszmidt/hctl/wiki/Manual#supported-formats) to see supported input and output formats

Jump [here](https://github.com/michalszmidt/hctl/issues/1) to feature-request issue.

# [Usage](https://github.com/michalszmidt/hctl/wiki/Usage)


```shell
# hctl -D -h
Manipulate on domains

Usage: hctl {domain|--domain|-D} [OPTIONS]

Options:
  -p, --path <path>          Path to file to be read [path without quotes]
  -o, --out <out>            Path to the out file [stdout/path without quotes]
  -z, --optimize <optimize>  Optimize for memory or speed, default: memory [memory/speed]
  -c, --config <config>      Path to config [without qoutes]
  -m, --mode <mode>          Process single or multiple lists [single/folder/config]
  -i, --intro <intro>        Whether append intro "Assembled From:..." to out file from config [true/false]
  -r, --rejected <rejected>  Whether save rejected to ./rejected.txt [true/false]
  -f, --format <format>      Type of out format [dnsmasq/loopback/empty/linewise/bind/snort/unbound/machosts/hostperm1/junkbuster/littlesnitch/pdnsd]
  -d, --dns <dns>            BETA: Validate your processed records issuing dns-over-tls query in predefined servers [yes/no]
  -h, --help                 Print help
```

# Magic behind the scene

There is heavy usage of rust iterators to enable lazy-reading

Processing is done by firstly removing addreses characteristic for host file like 127.0.0.1 and comments after the records. Next tool removes all whitespace characters and non-urls using regex.
See rules.rs if you want to embed similar solution in your app.

## License
[BSD-3-clause-no-military](https://github.com/michalszmidt/hctl/blob/main/LICENSE)

TD;LR
Modified [BSD-3-clause](https://choosealicense.com/licenses/bsd-3-clause-clear/) that does not allow usage for military purpose
