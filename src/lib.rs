extern crate futures;
extern crate grpcio;
extern crate protobuf;

use grpcio::{ChannelBuilder, EnvBuilder};
use std::sync::Arc;

pub mod api;
pub mod api_grpc;

pub fn new_client(addr: &str) -> ::api_grpc::DgraphClient {
    let env = Arc::new(EnvBuilder::new().build());
    let channel = ChannelBuilder::new(env).connect(addr);
    ::api_grpc::DgraphClient::new(channel)
}
