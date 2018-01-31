A rust client for dgraph
========================

[![CC-0](http://i.creativecommons.org/p/zero/1.0/88x31.png)](http://creativecommons.org/publicdomain/zero/1.0/)[![Build Status](https://travis-ci.org/davidB/dgraph_client-rs.svg?branch=master)](https://travis-ci.org/davidB/dgraph_client-rs)

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
# install [grpcio](https://crates.io/crates/grpcio)

# get api.proto & generate client
cd src
curl -O https://github.com/dgraph-io/dgraph/raw/v1.0.2/protos/api.proto
protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` api.proto

# build the lib
cd ..
cargo build
```