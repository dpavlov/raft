use actix::prelude::*;
use crate::model::{Node, State};
use crate::log::Operation;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Insert {
    key: String,
    value: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update {
    key: String,
    value: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Delete {
    key: String
}

impl Handler<Insert> for Node {
    type Result = ();

    fn handle(&mut self, msg: Insert, _: &mut Context<Self>) -> Self::Result {
        if self.state == State::LEADER {
            println!("[{} {}]: Insert op Key: {}, Value: {}!", self.id, self.state, msg.key, msg.value);
            let op = Operation::insert(self.current_term, msg.key, msg.value);
            self.log.append(op)
        }
    }
}

impl Handler<Update> for Node {
    type Result = ();

    fn handle(&mut self, msg: Update, _: &mut Context<Self>) -> Self::Result {
        if self.state == State::LEADER {
            println!("[{} {}]: Update op Key: {}, Value: {}!", self.id, self.state, msg.key, msg.value);
            let op = Operation::update(self.current_term, msg.key, msg.value);
            self.log.append(op)
        }
    }
}

impl Handler<Delete> for Node {
    type Result = ();

    fn handle(&mut self, msg: Delete, _: &mut Context<Self>) -> Self::Result {
        if self.state == State::LEADER {
            println!("[{} {}]: Delete op Key: {}!", self.id, self.state, msg.key);
            let op = Operation::delete(self.current_term, msg.key);
            self.log.append(op)
        }
    }
}