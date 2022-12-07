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

_The following can be viewed by running `discv5-cli --help`_

```bash
Simple CLI utility for creating and debugging discovery v5 servers

Usage: discv5-cli [OPTIONS] [COMMAND]

Commands:
  packet
          Performs packet operations
  request-enr
          Requests an ENR from a node
  server
          Runs a discv5 test server
  help
          Print this message or the help of the given subcommand(s)

Options:
  -v, --log-level <LOG_LEVEL>
          Sets the logging verbosity level.

          [default: info]

          Possible values:
          - trace: Trace level
          - debug: Debug level
          - info:  Info level
          - warn:  Warn level
          - error: Error level

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```

#### Server

The discv5-cli server provides the following options, which can be viewed by running `discv5-cli server --help`:

```bash
Runs a discv5 test server

Usage: discv5-cli server [OPTIONS]

Options:
  -l, --listen-address <LISTEN_ADDRESS>
          Specifies the listening address of the server. [default: 0.0.0.0]
  -p, --listen-port <LISTEN_PORT>
          Specifies the listening UDP port of the server. [default: 9000]
  -i, --enr-address <ENR_ADDRESS>
          Specifies the IP address of the ENR record. Not specifying this results in an ENR with no IP field, unless the -w switch is used.
  -u, --enr-port <ENR_PORT>
          Specifies the UDP port of the ENR record. Not specifying this results in an ENR with no UDP field, unless the -w switch is used.
  -q, --enr-seq-no <ENR_SEQ_NO>
          Specifies the ENR sequence number when creating the ENR.
  -d, --enr-eth2 <ENR_ETH2>
          Specifies the Eth2 field as ssz encoded hex bytes.
  -w, --enr-default
          The Enr IP address and port will be the same as the specified listening address and port.
  -k, --static-key
          Use a fixed static key (hard-coded). This is primarily for debugging.
  -t, --secp256k1-key <SECP256K1_KEY>
          Specify a secp256k1 private key (hex encoded) to use for the nodes identity.
  -e, --enr <ENR>
          A base64 ENR that this node will initially connect to.
  -n, --peer-update-min <PEER_UPDATE_MIN>
          The minimum number of peers required to update the IP address. Cannot be less than 2. [default: 2]
  -b, --break-time <BREAK_TIME>
          The time to wait between successive searches. Default is 10 seconds. [default: 10]
  -s, --stats <STATS>
          Displays statistics on the local routing table. [default: 10]
  -x, --no-search
          Prevents the server from doing any peer searches.
  -o, --bootstrap <BOOTSTRAP>
          Bootstraps the server peers from a specified file.
  -h, --help
          Print help information
```

In order to create an up-to-date `bootstrap.json` file, you can query a beacon-chain rpc using the `/eth/v1/node/peers` endpoint as specified in the [beacon-chain api](https://ethereum.github.io/beacon-APIs/). For example, run `curl http://0.0.0.0:3500/eth/v1/node/peers | jq` to get an output in the same format as [bootstrap.json](./bootstrap.json).

#### Packet

The discv5-cli packet provides the following options, which can be viewed by running `discv5-cli packet --help`:

```bash
Performs packet operations

Usage: discv5-cli packet <COMMAND>

Commands:
  decode  Decodes a packet
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help information
```

#### Request-ENR

The discv5-cli request-enr provides the following options, which can be viewed by running `discv5-cli request-enr --help`:

```bash
Requests an ENR from a node

Usage: discv5-cli request-enr --multiaddr <MULTIADDR>

Options:
  -m, --multiaddr <MULTIADDR>  The multiaddr of the node to request their ENR from
  -h, --help                   Print help information
```


## Examples


**Running a server**

```bash
$ discv5-cli server -e -KO4QMMX2IadOccPtAC29ueqrRzNEO5omCNRRTctz-QR20eyMwVR9yl6IDsoxmkg4naImuve8u2H6FO9fsmlXxso-suCAsKEZXRoMpBKJsWLAgAAAP__________gmlkgnY0gmlwhC0ftqSJc2VjcDI1NmsxoQI3vMd58jjnDZQm0KGQpbuWGSW9Bnon1GsF07XnlHna6YN0Y3CCekqDdWRwgsau --enr-port 50862 --enr-address 45.31.182.164 --break-time 5 --bootstrap bootstrap.json

2022-12-07T23:07:04.808Z INFO  [discv5_cli::server::node] Node Id: 0x773e..b498
2022-12-07T23:07:04.808Z INFO  [discv5_cli::server::node] Base64 ENR: enr:-IS4QArehERF1NJQn3zuGT5dAYBHNMjPCJCfGvs00AsMqyY6M80hoHZThbfveHbd-0GYqMy8mQiAAql5SlwpBRofc9IBgmlkgnY0gmlwhC0ftqSJc2VjcDI1NmsxoQL89kiZJgAhIf3hS6kAlou02R7IjayJWbULmBGTh52YUYN1ZHCCxq4
2022-12-07T23:07:04.808Z INFO  [discv5_cli::server::node] ip: 45.31.182.164, udp port:50862
2022-12-07T23:07:04.811Z INFO  [discv5_cli::server] Connecting to ENR. ip: Some(45.31.182.164), udp_port: Some(50862),  tcp_port: Some(31306)
2022-12-07T23:07:05.504Z INFO  [discv5_cli::server] Server listening on 0.0.0.0:9000
2022-12-07T23:07:05.505Z INFO  [discv5_cli::server] Query service running...
2022-12-07T23:07:05.505Z INFO  [discv5_cli::server::query] Searching for peers...
2022-12-07T23:07:05.505Z INFO  [discv5::service] Discv5 Service started
2022-12-07T23:07:05.505Z INFO  [discv5::service] Ip4
2022-12-07T23:07:11.152Z WARN  [discv5::service] NODES Response failed, but was partially processed from: Node: 0xb48c..4c2e, addr: 157.90.179.107:12000
2022-12-07T23:07:14.976Z INFO  [discv5_cli::server::query] Query Completed. Nodes found: 16
2022-12-07T23:07:14.976Z INFO  [discv5_cli::server::query] Node: 0xb48c..4c2e
2022-12-07T23:07:14.976Z INFO  [discv5_cli::server::query] Node: 0xb498..1ac2
2022-12-07T23:07:14.976Z INFO  [discv5_cli::server::query] Node: 0xb49a..f73c
2022-12-07T23:07:14.976Z INFO  [discv5_cli::server::query] Node: 0xb4a7..b1f0
2022-12-07T23:07:14.976Z INFO  [discv5_cli::server::query] Node: 0xb4b8..6f72
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb4d1..e58e
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb4e1..b19b
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb4e2..921e
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb4f5..566a
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb4f3..6657
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb4fe..49a9
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb4fa..ad8e
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb402..11d6
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb402..cc3c
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb402..6648
2022-12-07T23:07:14.977Z INFO  [discv5_cli::server::query] Node: 0xb403..b511
2022-12-07T23:07:14.978Z INFO  [discv5_cli::server::query] Bucket 251 statistics: Connected peers: 0 (Incoming: 0, Outgoing: 0), Disconnected Peers: 2
2022-12-07T23:07:14.978Z INFO  [discv5_cli::server::query] Bucket 252 statistics: Connected peers: 0 (Incoming: 0, Outgoing: 0), Disconnected Peers: 10
2022-12-07T23:07:14.978Z INFO  [discv5_cli::server::query] Bucket 253 statistics: Connected peers: 0 (Incoming: 0, Outgoing: 0), Disconnected Peers: 16
2022-12-07T23:07:14.979Z INFO  [discv5_cli::server::query] Bucket 255 statistics: Connected peers: 0 (Incoming: 0, Outgoing: 0), Disconnected Peers: 16
2022-12-07T23:07:14.979Z INFO  [discv5_cli::server::query] Bucket 256 statistics: Connected peers: 1 (Incoming: 0, Outgoing: 1), Disconnected Peers: 15
2022-12-07T23:07:14.979Z INFO  [discv5_cli::server::query] Bucket 247 statistics: Connected peers: 0 (Incoming: 0, Outgoing: 0), Disconnected Peers: 1
2022-12-07T23:07:14.979Z INFO  [discv5_cli::server::query] Bucket 254 statistics: Connected peers: 0 (Incoming: 0, Outgoing: 0), Disconnected Peers: 16
2022-12-07T23:07:14.979Z INFO  [discv5_cli::server::query] Bucket 248 statistics: Connected peers: 0 (Incoming: 0, Outgoing: 0), Disconnected Peers: 1
2022-12-07T23:07:15.668Z WARN  [discv5::service] NODES Response failed, but was partially processed from: Node: 0xb7c6..0a8c, addr: 185.49.111.250:30303
```

**Requesting an ENR**

```bash
$ discv5-cli request-enr /ip4/45.31.182.164/udp/31306/p2p/16Uiu2HAkyBFMsXS4Rpc3SfpaasFtLR6uKTnWqsUB8U8EMy24A5YU

2022-12-07 20:54:20,658 INFO [discv5::service] Discv5 Service started
2022-12-07 20:54:20,658 INFO [discv5::service] Ip4
2022-12-07 20:54:20,658 INFO [discv5_cli::request_enr] Requesting ENR for: /ip4/45.31.182.164/udp/31306/p2p/16Uiu2HAkyBFMsXS4Rpc3SfpaasFtLR6uKTnWqsUB8U8EMy24A5YU
2022-12-07 20:54:20,908 INFO [discv5_cli::request_enr] ENR Found:
2022-12-07 20:54:20,908 INFO [discv5_cli::request_enr] Sequence No:1545
2022-12-07 20:54:20,908 INFO [discv5_cli::request_enr] NodeId:0x9f89..e7b4
2022-12-07 20:54:20,910 INFO [discv5_cli::request_enr] Libp2p PeerId:16Uiu2HAkyBFMsXS4Rpc3SfpaasFtLR6uKTnWqsUB8U8EMy24A5YU
2022-12-07 20:54:20,910 INFO [discv5_cli::request_enr] IP:45.31.182.164
2022-12-07 20:54:20,910 INFO [discv5_cli::request_enr] TCP Port:31306
2022-12-07 20:54:20,910 INFO [discv5_cli::request_enr] UDP Port:25674
2022-12-07 20:54:20,910 INFO [discv5_cli::request_enr] Known multiaddrs:
2022-12-07 20:54:20,910 INFO [discv5_cli::request_enr] /ip4/45.31.182.164/udp/25674/p2p/16Uiu2HAkyBFMsXS4Rpc3SfpaasFtLR6uKTnWqsUB8U8EMy24A5YU
2022-12-07 20:54:20,910 INFO [discv5_cli::request_enr] /ip4/45.31.182.164/tcp/31306/p2p/16Uiu2HAkyBFMsXS4Rpc3SfpaasFtLR6uKTnWqsUB8U8EMy24A5YU
2022-12-07 20:54:20,911 INFO [discv5::service] Discv5 Service shutdown
```

**Decoding a packet**

```bash
$ discv5-cli -- packet decode --packet 9fd3c9ced567380bd7f0b25b4e8cb250401f9654bb92990b36f0220b65e8c96b029c629d897d25ed861b6b80c002c0c910ce86ccdea0de1f5721efc3c2e01a2a7288d986074592d40e4b8eee5c44e8ae56ff8f8bda79fd8d3e899e36fdc07b24d74edc8ab189775f9aedd8918fd03f7f52d98b --nodeid d94f5e91dbd9c22221fc9d778347fca7b9f49745071199c2a8960073a98169d9

2022-12-07T20:08:20.265Z INFO  [discv5_cli::packet] Using decoding node id: 0xd94f..69d9
2022-12-07T20:08:20.265Z INFO  [discv5_cli::packet] Packet decoded: (Packet { iv: 212446919118329375467898179749707297360, header: PacketHeader { message_nonce: [86, 230, 107, 84, 45, 19, 246, 151, 176, 105, 149, 81], kind: Message { src_id: NodeId { raw: [238, 233, 154, 181, 126, 7, 103, 153, 11, 0, 123, 98, 184, 185, 96, 132, 106, 88, 170, 186, 22, 234, 9, 223, 97, 84, 167, 121, 21, 83, 116, 203] } } }, message: [212, 14, 75, 142, 238, 92, 68, 232, 174, 86, 255, 143, 139, 218, 121, 253, 141, 62, 137, 158, 54, 253, 192, 123, 36, 215, 78, 220, 138, 177, 137, 119, 95, 154, 237, 216, 145, 143, 208, 63, 127, 82, 217, 139] }, [159, 211, 201, 206, 213, 103, 56, 11, 215, 240, 178, 91, 78, 140, 178, 80, 100, 105, 115, 99, 118, 53, 0, 1, 0, 86, 230, 107, 84, 45, 19, 246, 151, 176, 105, 149, 81, 0, 32, 238, 233, 154, 181, 126, 7, 103, 153, 11, 0, 123, 98, 184, 185, 96, 132, 106, 88, 170, 186, 22, 234, 9, 223, 97, 84, 167, 121, 21, 83, 116, 203])
```
