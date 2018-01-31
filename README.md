A rust client for dgraph
========================

The client used the grpc channel & api of dgraph.

## What is dgraph ?

from [dgraph site]( https://dgraph.io):

> Dgraph is an open source, horizontally scalable and distributed graph database, providing ACID transactions, consistent replication and linearizable reads. It's built from ground up to perform for a rich set of queries. Being a native graph database, it tightly controls how the data is arranged on disk to optimize for query performance and throughput, reducing disk seeks and network calls in a cluster.

> Dgraph's goal is to provide Google production level scale and throughput, with low enough latency to be serving real time user queries, over terabytes of structured data. Dgraph supports GraphQL-like query syntax, and responds in JSON and Protocol Buffers over GRPC and HTTP.

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