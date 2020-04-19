use actix::prelude::*;
use std::cmp::min;
use rand::Rng;

use super::model::{Node, Peer};
use super::heartbeat::FollowerHeartbeatElapseLoop;
use std::collections::HashSet;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Bootstrap {
    pub peers: Vec<(u32, Addr<Node>)>
}

impl Handler<Bootstrap> for Node {
    type Result = ();

    fn handle(&mut self, msg: Bootstrap, ctx: &mut Context<Self>) -> Self::Result {
        self.network = msg.peers.into_iter()
            .filter(|node| {node.1 != ctx.address()})
            .collect();

        let total_peers = self.network.len();
        let keys:Vec<u32> = self.network.keys().map(|i| *i).collect();

        let mut exist_peers: HashSet<u32> = HashSet::new();
        while self.peers.len() < min(3, total_peers) {
            let index = rand::thread_rng().gen_range(0, total_peers);
            let id = keys[index];
            if !exist_peers.contains(&id) {
                self.peers.push(Peer {
                    id,
                    address: self.network[&keys[index]].clone(),
                    next_index: 0,
                    match_index: 0,
                    is_vote_granted: false,
                    is_vote_revoked: false,
                });
                exist_peers.insert(id);
            }
        }

        let peer_ids = self.peers.iter().map(|p| p.id).collect::<Vec<u32>>();
        println!("[{} {}]: Got a bootstrap message with {:?} peers!", self.id, self.state, peer_ids);

        ctx.address().do_send(FollowerHeartbeatElapseLoop {});

        return;
    }
}