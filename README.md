```
                                              __            __     __
                                             / /_   _____  / /_   / /
                                            / __ \ / ___/ / __/  / / 
                                           / / / // /__  / /_   / /  
                                          /_/ /_/ \___/  \__/  /_/   
```
<div>
  <img alt="" src="https://img.shields.io/github/repo-size/michalszmidt/hctl" />
  <img alt="GitHub Last Commit" src="https://img.shields.io/github/last-commit/michalszmidt/hctl" />
  <img alt="GitHub Issues" src="https://img.shields.io/github/issues/michalszmidt/hctl" />
  <img alt="GitHub Pull Requests" src="https://img.shields.io/github/issues-pr/michalszmidt/hctl" />
</div>

## CI/CD
[![Build Status](https://api.cirrus-ci.com/github/michalszmidt/hctl.svg)](https://cirrus-ci.com/github/michalszmidt/hctl)

<!--
## CI/CD:
### Builds
- <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/publish_freebsd_amd64.yml"> FreeBSD [amd64]
- <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/publish_dragonflybsd_amd64.yml"> DragonflyBSD [amd64]
- <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/publish_netbsd_amd64.yml"> NetBSD [amd64]
- <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/publish_openbsd_amd64.yml"> OpenBSD [amd64]
- <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/publish_linux_musl_amd64.yml"> Linux musl [amd64]
- <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/publish_supported.yml"> Linux glibc [amd64]
- <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/publish_supported.yml"> MacOS X [amd64]
- <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/michalszmidt/hctl/publish_supported.yml"> Windows [amd64]
-->



<!--
|   | amd64 | aarch64 |
|---|---|---|---------|
| Linux glibc |   |   |
| Linux musl|   |   | 
| FreeBSD |   |   |
| MacOS X |   |   |
| DragonflyBSD |   | N/A |
| OpenBSD |   |   |
| NetBSD |   |   |
| Windows |   | N/|
-->

## About

Hostlists tools cli `hctl` is cli tool written in rust to manage (merge, transform, clear from comments etc.) your hostlists from diffrent formats to selected by you!

## Features

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
- [ ] Record existance validation from DNS servers (unencrypted and over tls)
- [ ] Whitelists (enumerated, from external lists file/https)
- [ ] Deep analysis (eliminating subdomains)
- [ ] Fault-tolerance, better error handling
- [ ] Deep rejected analysis

Jump [here](https://github.com/michalszmidt/hctl/issues/1) to feature-request issue.

## Magic behind the scene

There is heavy usage of rust iterators to enable lazy-reading

Processing is done by firstly removing addreses characteristic for host file like 127.0.0.1 and comments after the records. Next tool removes all whitespace characters and non-urls using regex.
See rules.rs if you want to embed similar solution in your app.

## Wiki

- [Home](https://github.com/michalszmidt/hctl/wiki)
- [Usage](https://github.com/michalszmidt/hctl/wiki/Usage)
- [Installation](https://github.com/michalszmidt/hctl/wiki/Installation)

## License
[BSD-3-clause-no-military](https://github.com/michalszmidt/hctl/blob/main/LICENSE)

TD;LR
Modified [BSD-3-clause](https://choosealicense.com/licenses/bsd-3-clause-clear/) that does not allow usage for military purpose