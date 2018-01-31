A rust client for dgraph
========================

[![License: CC-0 1.0](https://img.shields.io/badge/License-CC--0%201.0-lightgrey.svg)](http://creativecommons.org/publicdomain/zero/1.0/)[![Build Status](https://travis-ci.org/davidB/dgraph_client-rs.svg?branch=master)](https://travis-ci.org/davidB/dgraph_client-rs)[![dgraph_client’s current version badge](https://img.shields.io/crates/v/dgraph_client.svg)](https://crates.io/crates/dgraph_client)

The client used the grpc channel & api of dgraph.

## What is dgraph ?

from [dgraph site]( https://dgraph.io):

> Dgraph is an open source, horizontally scalable and distributed graph database, providing ACID transactions, consistent replication and linearizable reads. It's built from ground up to perform for a rich set of queries. Being a native graph database, it tightly controls how the data is arranged on disk to optimize for query performance and throughput, reducing disk seeks and network calls in a cluster.

> Dgraph's goal is to provide Google production level scale and throughput, with low enough latency to be serving real time user queries, over terabytes of structured data. Dgraph supports GraphQL-like query syntax, and responds in JSON and Protocol Buffers over GRPC and HTTP.

## Samples

see [examples](examples)

### examples/dgraph_tuto01
Sample adpated from go sample at https://docs.dgraph.io/clients/

```sh
# launch dgraph (grpc on localhost:9080)
# see doc of dgraph

# run
cargo run --example dgraph_tuto01
```

## Build

The lib used [grpcio](https://crates.io/crates/grpcio) to generate the rust code from grpc's proto file.

```sh
cargo build
```

### Update the grpc client

```sh
# install [grpcio](https://crates.io/crates/grpcio)

# get api.proto & generate client
cd src
curl -O https://github.com/dgraph-io/dgraph/raw/v1.0.2/protos/api.proto
protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` api.proto

```