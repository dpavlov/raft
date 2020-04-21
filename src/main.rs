mod bootstrap;
mod node;
mod client;
mod model;
mod heartbeat;
mod election;
mod log;
mod ops;

use actix::prelude::*;
use model::{Node, Client};
use bootstrap::Bootstrap;
use std::time::Duration;

fn main() {
    let system = System::new("raft-cluster");

    let nodes_count: u32 = 3;

    let mut nodes: Vec<(u32, Addr<Node>)> = Vec::with_capacity(nodes_count as usize);
    for node_id in 1 .. (nodes_count + 1) {
        let node = Node::create(|_ctx| { Node::new(node_id) } );
        nodes.push((node_id, node));
    }

    for (_, node) in &nodes {
        node.do_send(Bootstrap {peers: nodes.clone()});
    }

    Client::create(|_ctx| { Client::new(nodes.iter().map(|n| n.1.clone() ).collect()) } );

    let _ = system.run();
}
