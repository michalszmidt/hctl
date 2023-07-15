# Hostlists tools cli (hctl)
## About

This is simple cli tool written in rust to manage (transform, clear from comments etc.) your hostlists

## Features

- [x] Merge hostlist
- [x] Remove duplicates
- [x] Remove commented lines
- [x] Fetch lists directly from network (https)
- [x] Parallel processing
- [x] Progressbar
- [x] Rejected lines may be saved with information about source list
- [x] Support for Windows encoded text files (line ending with \n\r)
- [x] Supported input formats (linewise, 127.0.0.1, 0.0.0.0)
- [ ] Supported output formats (linewise, 127.0.0.1, 0.0.0.0, dnsmasq)
- [ ] Whitelists
- [ ] Fault-tolerance, better error handling
- [ ] Deep rejected analysis

Jump [here](https://github.com/michalszmidt/hctl/issues/1) to feature-request issue.

## Magic behind 

There is heavy usage of rust iterators to enable lazy-reading

Processing is done by firstly removing addreses characteristic for host file like 127.0.0.1 and comments after the records. Next tool removes all whitespace characters and non-urls using regex.
See rules.rs if you want to embed similar solution in your app.

## Wiki

- [Home](https://github.com/michalszmidt/hctl/wiki)
- [Usage](https://github.com/michalszmidt/hctl/wiki/Usage)
- [Installation](https://github.com/michalszmidt/hctl/wiki/Installation)

[BSD-3-clause-no-military](https://github.com/michalszmidt/hctl/blob/main/LICENSE)