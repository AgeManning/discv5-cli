discv5-cli
============

[![Build Status]][Build Link] [![Doc Status]][Doc Link] [![Crates
Status]][Crates Link]

[Build Status]: https://github.com/AgeManning/discv5-cli/workflows/build/badge.svg?branch=master
[Build Link]: https://github.com/AgeManning/discv5-cli/actions
[Doc Status]: https://docs.rs/discv5-cli/badge.svg
[Doc Link]: https://docs.rs/discv5-cli
[Crates Status]: https://img.shields.io/crates/v/discv5-cli.svg
[Crates Link]: https://crates.io/crates/discv5-cli

[Documentation at docs.rs](https://docs.rs/discv5-cli)

## Overview

This is a simple CLI utility for creating and debugging discovery v5 servers.

## Install

This can be installed via cargo:

```bash
$ cargo install discv5-cli --version 0.1.0-alpha
```

## Usage

```bash
Sigma Prime <contact@sigmaprime.io>
Simple CLI tool for starting and debugging discv5 servers. This currently runs a discv5 server which regularly performs
peer search queries

USAGE:
    discv5-cli [FLAGS] [OPTIONS] --listen-address <IP-ADDRESS> --listen-port <PORT>

FLAGS:
    -w                  The Enr IP address and port will be the same as the specified listening address and port.
    -h, --help          Prints help information
    -k, --static-key    Use a fixed static key (hard-coded). This is primarily for debugging.
    -V, --version       Prints version information

OPTIONS:
    -e, --enr <BASE64-ENR>               A base64 ENR that this node will initially connect to.
        --enr-address <IP-ADDRESS>       Specifies the IP address of the ENR record. Not specifying this results in an
                                         ENR with no IP field, unless the -w switch is used.
        --enr-port <PORT>                Specifies the UDP port of the ENR record. Not specifying this results in an ENR
                                         with no UDP field, unless the -w switch is used.
    -l, --listen-address <IP-ADDRESS>    Specifies the listening address of the server. [default: 127.0.0.1]
    -p, --listen-port <PORT>             Specifies the listening UDP port of the server. [default: 9000]
```

## Example

```bash
$ discv5-cli -l 127.0.0.1 -p 9001 -w -e -IS4QCs0BSKEvnX8om4rAAi7D2p2lwQ7LVpAeESY2ikm1b5dBOqJC7istWMVg06dy-I09C8NuZdodEFNxIiiolWwSWkBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCIyg
Node Id: 0x98b6..42f9
Peer Id: QmQJbNYRuLJo87Q77ZAj246NfPtctKsBbWtnTvJDLrGcw1
Base64 ENR: enr:-IS4QKKLuzCDjV4hjA2-77cO0Xjx5vLGODQKe_Fl6Qim8qHBMN4chtaLqv6Xz6BWv5hfVvn0d2G0dt94ZAG9OHska44BgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQLHT5xE1ZhCXfrOv3D66tPfDZxV57TaX0rky3uwRhyBe4N1ZHCCIyk
ip: 127.0.0.1, udp port:9001
Connecting to ENR. ip: Some(127.0.0.1), udp_port: Some(9000),  tcp_port: None
Query Completed. Nodes found:
PeerId: QmdFQ2frpcyD3DiaMcM6XP37X3xCAV81GoB3jRusAa7kxu
Connected Peers: 1
Searching for peers...
Query Completed. Nodes found:
PeerId: QmdFQ2frpcyD3DiaMcM6XP37X3xCAV81GoB3jRusAa7kxu
```
