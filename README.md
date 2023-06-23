# Hostlists tools cli (hctl)
## About

This is simple cli tool written in rust to manage (transform, clear from comments etc.) your hostlists

## Features

(empty boxes means WIP or Planned, Jump here to feature-request issue).

- [x] Merge hostlist
- [x] Remove duplicates
- [x] Remove commented lines
- [x] Fetch lists directly from network (https)
- [x] Parallel processing
- [x] Progressbar
- [ ] Fault-tolerance
- [ ] Dnsmasq format
- [ ] Snort format

## Logic

*Mutual:*

There is heavy usage of rust iterators to enable lazy-reading

*Domains:*

Processing is done by firstly removing addreses characteristic for host file like 127.0.0.1 then removing all non-urls using regex.
See regex.rs if you want to embed similar solution in your app.

## Wiki

Jump here

## License

BSD-3-clause-no-military