use actix::prelude::*;
use rand::Rng;
use futures::prelude::*;

use super::model::{Client, Node};
use std::time::Duration;
use crate::node::LeaderDiscovered;
use crate::log::OpType;
use crate::ops::{Insert, Update, Delete, OpResult};



#[derive(Message)]
#[rtype(result = "()")]
pub struct DiscoverLeader {
    pub client: Addr<Client>
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OpsLoop {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaderCheckLoop {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaderLost {}

impl Actor for Client {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("[CLIENT] Client has been started with {} nodes cluster", self.cluster.len());
        ctx.run_interval(Duration::from_millis(1000), move |_act, ctx| {
            ctx.address().do_send(LeaderCheckLoop{});
        });
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            ctx.address().do_send(OpsLoop{})
        });
    }
}

impl Handler<LeaderCheckLoop> for Client {
    type Result = ();

    fn handle(&mut self, _: LeaderCheckLoop, ctx: &mut Context<Self>) -> Self::Result {
        if self.leader.is_none() {
            println!("[CLIENT] There is no leader yet. Discovering...");
            for node in &self.cluster {
                node.do_send(DiscoverLeader{ client: ctx.address().clone() } )
            }
        }
    }
}

impl Handler<LeaderDiscovered> for Client {
    type Result = ();

    fn handle(&mut self, msg: LeaderDiscovered, ctx: &mut Context<Self>) -> Self::Result {
        println!("[CLIENT] Leader {} has been discovered", msg.id);
        self.leader = Some(msg.leader);;
    }
}

impl Handler<LeaderLost> for Client {
    type Result = ();

    fn handle(&mut self, _: LeaderLost, ctx: &mut Context<Self>) -> Self::Result {
        println!("[CLIENT] Leader lost");
        self.leader = None;
    }
}

impl Handler<OpsLoop> for Client {
    type Result = ();

    fn handle(&mut self, _: OpsLoop, ctx: &mut Context<Self>) -> Self::Result {
        if let Some(leader) = &self.leader {
            println!("[CLIENT] Sending op to leader");
            let op_index = rand::thread_rng().gen_range(0, 3);


            let key_index = rand::thread_rng().gen_range(0, 5);
            let key = ["a", "b", "c", "d", "e"][key_index].to_string();

            let val_index = rand::thread_rng().gen_range(0, 5);
            let value = ["1", "2", "3", "4", "5"][val_index].to_string();

            let client = ctx.address();

            match op_index {
                0 => {
                    let req: Request<Node, Insert> = leader.send(Insert{key, value});
                    Arbiter::spawn(req.map(move |ops| {
                        match ops.unwrap() {
                            OpResult::Applied => println!("[CLIENT] Insert applied to leader"),
                            OpResult::NotLeader => client.do_send(LeaderLost{})
                        }
                    }));
                },
                1 => {
                    let req: Request<Node, Update> = leader.send(Update{key, value});
                    Arbiter::spawn(req.map(move |ops| {
                        match ops.unwrap() {
                            OpResult::Applied => println!("[CLIENT] Update applied to leader"),
                            OpResult::NotLeader => client.do_send(LeaderLost{})
                        }
                    }));
                },
                _ => {
                    let req: Request<Node, Delete> = leader.send(Delete{key});
                    Arbiter::spawn(req.map(move |ops| {
                        match ops.unwrap() {
                            OpResult::Applied => println!("[CLIENT] Delete applied to leader"),
                            OpResult::NotLeader => client.do_send(LeaderLost{})
                        }
                    }));
                },
            };
        }
    }
}