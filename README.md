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
$ cargo install discv5-cli
```

## Usage

```
discv5-cli 0.2.5
Sigma Prime <contact@sigmaprime.io>
Simple CLI tool for starting and debugging discv5 servers and packets. This currently runs a discv5 server which
regularly performs peer search queries.

USAGE:
    discv5-cli [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -v, --log-level <level>    Specifies the listening address of the server. [default: info]  [possible values: trace,
                               debug, info, warn, error]

SUBCOMMANDS:
    help           Prints this message or the help of the given subcommand(s)
    packet         Performs various packet encoding/decoding functions
    request-enr    Requests the ENR of a multiaddr
    server         Runs a discv5 test server
```

## Examples


**Running a server**

```bash
$ discv5-cli server -l 127.0.0.1 -p 9001 -w -e -IS4QCs0BSKEvnX8om4rAAi7D2p2lwQ7LVpAeESY2ikm1b5dBOqJC7istWMVg06dy-I09C8NuZdodEFNxIiiolWwSWkBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQPKY0yuDUmstAHYpMa2_oxVtw0RW_QAdpzBQA8yWM0xOIN1ZHCCIyg

2020-05-20 21:15:06,968 INFO  [discv5_cli::server] Node Id: 0x62e5..2ca7
2020-05-20 21:15:06,968 INFO  [discv5_cli::server] Base64 ENR: enr:-IS4QKzeZIMc6NtdIYGTQvo0Q2Aw0NicbQrW7zArKWU6iNebWMuBTjJnes8nBJ-wAua-W6XNatKBHrNhxFcaUSysVJwBgmlkgnY0gmlwhH8AAAGJc2VjcDI1NmsxoQIAN7xJ9ce7O7iw-dFrlOzj4sC1Y0Gvic5hb5Rxfs4Xt4N1ZHCCIyk
2020-05-20 21:15:06,968 INFO  [discv5_cli::server] ip: 127.0.0.1, udp port:9001
2020-05-20 21:15:06,969 INFO  [discv5_cli::server] Connecting to ENR. ip: Some(127.0.0.1), udp_port: Some(9000),  tcp_port: None
2020-05-20 21:15:06,971 INFO  [discv5_cli::server::query_server] Connected Peers: 0
2020-05-20 21:15:06,971 INFO  [discv5_cli::server::query_server] Searching for peers...
2020-05-20 21:15:10,973 INFO  [discv5_cli::server::query_server] Query Completed. No peers found.
```

**Requesting an ENR**

```bash
$ discv5-cli request-enr -m /ip4/127.0.0.1/udp/4000/p2p/16Uiu2HAm8WQDKEU4poRcVqfm4X2eLDZDMAnezYicLZV3gDTM3asN

2022-12-07T19:53:33.031Z INFO  [discv5::service] Discv5 Service started
2022-12-07T19:53:33.031Z INFO  [discv5::service] Ip4
2022-12-07T19:53:33.031Z INFO  [discv5_cli::request_enr] Requesting ENR for: /ip4/127.0.0.1/udp/4000/p2p/16Uiu2HAm8WQDKEU4poRcVqfm4X2eLDZDMAnezYicLZV3gDTM3asN
2022-12-07T19:53:34.038Z ERROR [discv5_cli::request_enr] Failed to obtain ENR. Error: Timeout
```

**Decoding a packet**

```bash
$ discv5-cli -- packet decode --packet 9fd3c9ced567380bd7f0b25b4e8cb250401f9654bb92990b36f0220b65e8c96b029c629d897d25ed861b6b80c002c0c910ce86ccdea0de1f5721efc3c2e01a2a7288d986074592d40e4b8eee5c44e8ae56ff8f8bda79fd8d3e899e36fdc07b24d74edc8ab189775f9aedd8918fd03f7f52d98b --nodeid d94f5e91dbd9c22221fc9d778347fca7b9f49745071199c2a8960073a98169d9

2022-12-07T20:08:20.265Z INFO  [discv5_cli::packet] Using decoding node id: 0xd94f..69d9
2022-12-07T20:08:20.265Z INFO  [discv5_cli::packet] Packet decoded: (Packet { iv: 212446919118329375467898179749707297360, header: PacketHeader { message_nonce: [86, 230, 107, 84, 45, 19, 246, 151, 176, 105, 149, 81], kind: Message { src_id: NodeId { raw: [238, 233, 154, 181, 126, 7, 103, 153, 11, 0, 123, 98, 184, 185, 96, 132, 106, 88, 170, 186, 22, 234, 9, 223, 97, 84, 167, 121, 21, 83, 116, 203] } } }, message: [212, 14, 75, 142, 238, 92, 68, 232, 174, 86, 255, 143, 139, 218, 121, 253, 141, 62, 137, 158, 54, 253, 192, 123, 36, 215, 78, 220, 138, 177, 137, 119, 95, 154, 237, 216, 145, 143, 208, 63, 127, 82, 217, 139] }, [159, 211, 201, 206, 213, 103, 56, 11, 215, 240, 178, 91, 78, 140, 178, 80, 100, 105, 115, 99, 118, 53, 0, 1, 0, 86, 230, 107, 84, 45, 19, 246, 151, 176, 105, 149, 81, 0, 32, 238, 233, 154, 181, 126, 7, 103, 153, 11, 0, 123, 98, 184, 185, 96, 132, 106, 88, 170, 186, 22, 234, 9, 223, 97, 84, 167, 121, 21, 83, 116, 203])
```
