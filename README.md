# ipinfo-cli

A simple command-line tool to lookup your own IP address, or get geolocation details for an IP.

```console
$ ipinfo 8.8.8.8
{
  "ip": "8.8.8.8",
  "hostname": "dns.google",
  "anycast": true,
  "city": "Mountain View",
  "region": "California",
  "country": "US",
  "loc": "37.4056,-122.0775",
  "org": "AS15169 Google LLC",
  "postal": "94043",
  "timezone": "America/Los_Angeles",
  "readme": "https://ipinfo.io/missingauth"
}

$ ipinfo ::1
{
  "ip": "::1",
  "bogon": true
}
```

Calling `ipinfo` with no argument will return information for your own IP address.

This tool relies on the [IPinfo.io](https://ipinfo.io) IP address API.
You can optionally provide an API token in the environment variable `IPINFO_TOKEN`.

This tool was developped for fun and learning.
It is not related and does not share code with the official [ipinfo crate](https://crates.io/crates/ipinfo).

## Installation

You may install `ipinfo-cli` locally by running

```console
$ cargo install --git https://github.com/vivienm/ipinfo-cli.git
```

### Autocompletion

To enable autocompletion in Bash, run

```console
$ source <(ipinfo --completion bash)
```
