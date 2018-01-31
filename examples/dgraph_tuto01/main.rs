// Sample adpated from https://docs.dgraph.io/clients/
extern crate dgraph_client;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate chrono;

mod data;

use slog::Drain;
use dgraph_client::api;
use chrono::prelude::*;
use data::*;
use std::collections::HashMap;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());

    info!(log, "connect to dgraph via grpc at localhost:9080");
    let client = dgraph_client::new_client("localhost:9080");

    // setup

    // Drop all data including schema from the dgraph instance. This is useful
    // for small examples such as this, since it puts dgraph into a clean
    // state.
    info!(log, "drop all");
    let mut op_cleanup = api::Operation::new();
    op_cleanup.drop_all = true;
    client.alter(&op_cleanup).expect("drop schema");

    // Install a schema into dgraph. Accounts have a `name` and a `balance`.
    info!(log, "setup schema");
    let mut op_schema = api::Operation::new();
    op_schema.schema = r#"
        name: string @index(exact) .
        age: int .
        married: bool .
        loc: geo .
        dob: datetime .
    "#.to_string();
    client.alter(&op_schema).expect("set schema");

    info!(log, "push data");
    let dob = Utc.ymd(1980, 1, 1).and_hms(23, 0, 0);
    // While setting an object if a struct has a Uid then its properties in the graph are updated
    // else a new node is created.
    // In the example below new nodes for Alice, Bob and Charlie and school are created (since they
    // dont have a Uid).
    let p = Person {
        name: "Alice".to_string(),
        age: Some(26),
        married: Some(true),
        location: Some(Location {
            kind: "Point".to_string(),
            coordinates: vec![1.1f64, 2f64],
        }),
        dob: Some(dob),
        //raw: "raw_bytes".as_bytes().to_vec(),
        friends: Some(vec![
            Person {
                name: "Bob".to_string(),
                age: Some(24),
                ..Default::default()
            },
            Person {
                name: "Charlie".to_string(),
                age: Some(29),
                ..Default::default()
            },
        ]),
        school: Some(vec![
            School {
                name: "Crown Public School".to_string(),
            },
        ]),
        ..Default::default()
    };
    let mut mutation = api::Mutation::new();
    mutation.commit_now = true;
    mutation.set_json = serde_json::to_vec(&p).expect("valid json");
    let assigned = client.mutate(&mutation).expect("set data");
    //info!(log, format!("mutation result: {:?}", mutation_txn));

    info!(log, "query");
    let mut req = ::api::Request::new();

    // Assigned uids for nodes which were created would be returned in the resp.AssignedUids map.
    req.vars = [("$id".to_string(), assigned.uids["blank-0"].clone())]
        .iter()
        .cloned()
        .collect::<HashMap<String, String>>();
    req.query = r#"query Me($id: string){
        me(func: uid($id)) {
            name
            dob
            age
            loc
            raw_bytes
            married
            friend @filter(eq(name, "Bob")){
                name
                age
            }
            school {
                name
            }
        }
    }"#.to_string();
    let resp = client.query(&req).expect("query");
    println!("{:?}", resp);
    let root: Root = serde_json::from_slice(&resp.json).expect("parsing");
    // output: Root { me: [Person { name: "Alice", age: Some(26), dob: Some(1980-01-01T23:00:00Z), married: Some(true), raw: None, friends: Some([Person { name: "Bob", age: Some(24), dob: None, married: None, raw: None, friends: None, location: None, school: None }]), location: Some(Location { kind: "Point", coordinates: [1.1, 2.0] }), school: Some([School { name: "Crown Public School" }]) }] }
    println!("{:?}", root);
    //info!(log, "resp", "root" => root);
    // info!(log, "close client");
    // client.close();
}
