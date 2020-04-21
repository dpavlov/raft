use actix::prelude::*;
use crate::model::{Node, State};
use crate::log::Operation;

pub enum OpResult {
    Applied,
    NotLeader
}

#[derive(Message)]
#[rtype(result = "OpResult")]
pub struct Insert {
    pub key: String,
    pub value: String,
}

#[derive(Message)]
#[rtype(result = "OpResult")]
pub struct Update {
    pub key: String,
    pub value: String,
}

#[derive(Message)]
#[rtype(result = "OpResult")]
pub struct Delete {
    pub key: String
}

impl Handler<Insert> for Node {
    type Result = MessageResult<Insert>;

    fn handle(&mut self, msg: Insert, _: &mut Context<Self>) -> Self::Result {
        if self.state == State::LEADER {
            println!("[{} {}]: Insert op Key: {}, Value: {}!", self.id, self.state, msg.key, msg.value);
            let op = Operation::insert(self.current_term, msg.key, msg.value);
            self.log.append(op);
            MessageResult(OpResult::Applied)
        } else {
            MessageResult(OpResult::NotLeader)
        }
    }
}

impl Handler<Update> for Node {
    type Result = MessageResult<Update>;

    fn handle(&mut self, msg: Update, _: &mut Context<Self>) -> Self::Result {
        if self.state == State::LEADER {
            println!("[{} {}]: Update op Key: {}, Value: {}!", self.id, self.state, msg.key, msg.value);
            let op = Operation::update(self.current_term, msg.key, msg.value);
            self.log.append(op);
            MessageResult(OpResult::Applied)
        } else {
            MessageResult(OpResult::NotLeader)
        }
    }
}

impl Handler<Delete> for Node {
    type Result = MessageResult<Delete>;

    fn handle(&mut self, msg: Delete, _: &mut Context<Self>) -> Self::Result {
        if self.state == State::LEADER {
            println!("[{} {}]: Delete op Key: {}!", self.id, self.state, msg.key);
            let op = Operation::delete(self.current_term, msg.key);
            self.log.append(op);
            MessageResult(OpResult::Applied)
        } else {
            MessageResult(OpResult::NotLeader)
        }
    }
}