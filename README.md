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

# Downloads
## [From release page](https://github.com/michalszmidt/hctl/releases/latest)

<table>
<tr><th>Platform</th><th>Downloads</th><th>Build Status</th></tr>
<tr><td>

| OS |
|----------|
| Linux glibc |
| Linux musl  |
| MacOS X |
| FreeBSD |
| DragonflyBSD |
| OpenBSD |
| NetBSD |
| Windows |

</td><td>
    
| amd64 | aarch64 |
|-------|---------|
|[app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-linux-glibc-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-linux-glibc-amd64) | [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-linux-glibc-aarch64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-linux-glibc-aarch64) |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-linux-musl-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-linux-musl-amd64) | [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-linux-musl-aarch64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-linux-musl-aarch64) |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-macosx-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-macosx-amd64) | [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-macosx-silicon), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-macosx-silicon) |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-freebsd-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-freebsd-amd64) | NSU |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-dragonflybsd-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-dragonflybsd-amd64) | N/A |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-openbsd-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-openbsd-amd64) | NSU |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-netbsd-amd64), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-netbsd-amd64) | NSU |
| [app](https://github.com/michalszmidt/hctl/releases/download/latest/hctl-windows-amd64.exe), [sha256](https://github.com/michalszmidt/hctl/releases/download/latest/sha256-hctl-windows-amd64.exe.txt) | WSU |

</td><td>

| amd64 | aarch64 |
|-------|---------|
| [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=relese_linux_glibc_amd64)](https://cirrus-ci.com/github/michalszmidt/hctl) | [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=relese_linux_glibc_aarch64)](https://cirrus-ci.com/github/michalszmidt/hctl) |
| [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=relese_linux_musl_aarch64)](https://cirrus-ci.com/github/michalszmidt/hctl) | [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=relese_linux_musl_amd64)](https://cirrus-ci.com/github/michalszmidt/hctl) |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_macosx_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=relese_macosx_silicon)](https://cirrus-ci.com/github/michalszmidt/hctl) |
| [![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg?task=relese_freebsd_amd64)](https://cirrus-ci.com/github/michalszmidt/hctl) | NSU |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_dragonflybsd_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | N/A |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_openbsd_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | NSU |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_netbsd_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | NSU |
| [![Build Status](https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/release_windows_amd64.yml)](https://github.com/michalszmidt/hctl/actions) | WSU |

</td></tr> </table>


- NSU - Not Set Up Yet, will appear in future
- WSU - Won't set up for some reason

# Features

- [x] Merge hostlist
- [x] Remove duplicates
- [x] Remove commented lines (default behaviour)
- [x] Fetch lists directly from network (https)
- [x] YAML config
- [x] Parallel processing
- [x] Progressbar
- [x] Rejected lines may be saved with information about source list
- [x] Support for Windows encoded text files (line ending with \n\r)
- [x] Supported input formats (linewise, 127.0.0.1, 0.0.0.0, ~dnsmasq~, ~snort~, ~squid~, ~unbound~, ~bind~, ~netgar~, ~adblock~)
- [x] Supported output formats (linewise, 127.0.0.1, 0.0.0.0, dnsmasq, ~snort~, ~squid~, ~unbound~, ~bind~, ~netgar~, ~adblock~)
- [x] Output to stdout to be used with unix pipe stream control
- [x] Whitelists (enumerated, from external lists file/https)
- [x] Whitelists including subdomains
- [ ] Input from stdout to be used with unix pipe stream control
- [ ] YAML has full settings (remote sources, ~whitelist~ , ~args from cli~)
- [ ] Subdomains family recognition, save with subdomain order (like adaway does)
- [ ] Record existance validation from DNS servers (unencrypted and over tls)
- [ ] Fault-tolerance, better error handling
- [ ] Deep rejected analysis
- [ ] Quiet option

Jump [here](https://github.com/michalszmidt/hctl/issues/1) to feature-request issue.

# Magic behind the scene

There is heavy usage of rust iterators to enable lazy-reading

Processing is done by firstly removing addreses characteristic for host file like 127.0.0.1 and comments after the records. Next tool removes all whitespace characters and non-urls using regex.
See rules.rs if you want to embed similar solution in your app.

## License
[BSD-3-clause-no-military](https://github.com/michalszmidt/hctl/blob/main/LICENSE)

TD;LR
Modified [BSD-3-clause](https://choosealicense.com/licenses/bsd-3-clause-clear/) that does not allow usage for military purpose