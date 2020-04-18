use actix::prelude::*;

struct Node {
 id: u32,
 peers: Vec<Addr<Node>>
}

#[derive(Message)]
#[rtype(result = "()")]
struct Bootstrap {
    peers: Vec<Addr<Node>>
}

#[derive(Message)]
#[rtype(result = "()")]
struct Hello {
    from: u32
}

impl Actor for Node {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("Node {} has been started", self.id);
    }
}

impl Handler<Bootstrap> for Node {
    type Result = ();

    fn handle(&mut self, msg: Bootstrap, ctx: &mut Context<Self>) -> Self::Result {
        self.peers = msg.peers.into_iter()
            .filter(|addr| {*addr != ctx.address()})
            .collect();

        println!("[{}]: Got a bootstrap message with {} peers!", self.id, self.peers.len());

        for peer in &self.peers {
            peer.do_send(Hello{from: self.id})
        }

        return;
    }
}

impl Handler<Hello> for Node {
    type Result = ();

    fn handle(&mut self, msg: Hello, _: &mut Context<Self>) -> Self::Result {
        println!("[{}]: Got hello message from node {}!", self.id, msg.from);
        return;
    }
}

fn main() {
    let system = System::new("raft-cluster");

    let nodes_count: u32 = 2;

    let mut nodes: Vec<Addr<Node>> = Vec::with_capacity(nodes_count as usize);
    for node_id in 1 .. (nodes_count + 1) {
        let node = Node::create(|_ctx| {
            Node { id: node_id, peers: vec![] }
        });
        nodes.push(node);
    }

    for node in &nodes {
        node.do_send(Bootstrap {peers: nodes.clone()});
    }

    let _ = system.run();
}
