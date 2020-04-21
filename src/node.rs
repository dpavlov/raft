use actix::prelude::*;

use super::client::DiscoverLeader;
use super::model::{Node, State};

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaderDiscovered {
    pub id: u32,
    pub leader: Addr<Node>,
}


impl Actor for Node {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("[{} {}]: Node has been started", self.id, self.state);
    }
}

impl Handler<DiscoverLeader> for Node {
    type Result = ();

    fn handle(&mut self, msg: DiscoverLeader, ctx: &mut Context<Self>) -> Self::Result {
        println!("[{} {}]: Discovering leader request", self.id, self.state);
        if self.state == State::LEADER {
            msg.client.do_send(LeaderDiscovered {
                id: self.id,
                leader: ctx.address()
            })
        }
    }
}