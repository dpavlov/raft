use std::collections::HashMap;
use actix::prelude::*;
use std::fmt::{Display, Formatter, Result};
use super::log::OperationLog;
use std::time::SystemTime;

#[derive(Eq, PartialEq)]
pub enum State {
    LEADER,
    CANDIDATE,
    FOLLOWER
}

pub struct Storage {
    pub store: HashMap<String, String>
}

pub struct Peer {
    pub id: u32,
    pub address: Addr<Node>,
    pub next_index: u32,
    pub match_index: u32,
    pub is_vote_granted: bool,
    pub is_vote_revoked: bool,
}

pub struct Node {
    pub id: u32,
    pub network: HashMap<u32, Addr<Node>>,
    pub state: State,
    pub heartbeat: u128,
    pub voted_for: u32,
    pub current_term: i64,
    pub commit_index: i32,
    pub peers: Vec<Peer>,
    pub log: OperationLog,
    pub storage: Storage
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let name = match self {
            State::LEADER => "LEADER",
            State::FOLLOWER => "FOLLOWER",
            State::CANDIDATE => "CANDIDATE",
        };
        write!(f, "{}", name)
    }
}

impl Node {

    pub fn new(id: u32) -> Node {
        Node {
            id,
            network: HashMap::new(),
            state: State::FOLLOWER,
            heartbeat: 0,
            voted_for: 0,
            current_term: 0,
            commit_index: -1,
            peers: vec![],
            log: OperationLog::new(),
            storage: Storage {
                store: HashMap::new()
            }
        }
    }

    pub fn reset_election(&mut self) {
        self.state = State::CANDIDATE;
        self.voted_for = 0;
        for pear in &mut self.peers {
            pear.is_vote_granted = false;
            pear.is_vote_revoked = false;
        }
    }

    pub fn reset_heartbeat(&mut self) {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_millis();
        self.heartbeat = now
    }

}

pub struct Client {
    pub cluster: Vec<Addr<Node>>,
}

impl Client {
    pub fn new(nodes: Vec<Addr<Node>>) -> Self {
        Client { cluster: nodes }
    }
}