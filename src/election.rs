use actix::prelude::*;
use std::time::SystemTime;

use super::model::{Node, State};
use super::log::Operation;
use super::heartbeat::FollowerHeartbeatElapseLoop;
use crate::heartbeat::LeaderHeartbeatSendLoop;


#[derive(Message)]
#[rtype(result = "()")]
pub struct NewElectionRound {}

#[derive(Message)]
#[rtype(result = "()")]
struct VoteFor {
    candidate_id: u32,
    address: Addr<Node>,
    term: i64,
    last_log_index: i32,
    last_log_term: i64,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Voted {
    id: u32,
    term: i64,
    vote_granted: bool,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Append {
    pub id: u32,
    pub term: i64,
    pub ops: Vec<Operation>
}

impl Handler<NewElectionRound> for Node {
    type Result = ();

    fn handle(&mut self, _: NewElectionRound, ctx: &mut Context<Self>) -> Self::Result {
        if self.voted_for == 0 {
            self.current_term += 1;
            println!("[{} {}]: New election round for term {}", self.id, self.state, self.current_term);
            self.voted_for = self.id;
            for peer in &self.peers {
                println!("[{} {}]: Sending vote request to peer {} with term {}", self.id, self.state, peer.id, self.current_term);
                peer.address.do_send(
                    VoteFor {
                        candidate_id: self.id,
                        address: ctx.address(),
                        term: self.current_term,
                        last_log_index: -1,
                        last_log_term: -1,
                    }
                )
            }
        } else {
            println!("[{} {}]: Cancel election! Already voted for another candidate {}!", self.id, self.state, self.voted_for);
            self.reset_heartbeat();
            ctx.address().do_send(FollowerHeartbeatElapseLoop {})
        }
    }
}

impl Handler<VoteFor> for Node {
    type Result = ();

    fn handle(&mut self, msg: VoteFor, ctx: &mut Context<Self>) -> Self::Result {
        if self.current_term > msg.term {
            println!("[{} {}]: Vote request from candidate {} rejected [{} > {}] !", self.id, self.state, msg.candidate_id, self.current_term, msg.term);
            msg.address.do_send(Voted {
                id: self.id,
                term: self.current_term,
                vote_granted: false,
            })
        } else {
            println!("[{} {}]: Vote request from candidate {} with term {}!", self.id, self.state, msg.candidate_id, msg.term);
            let term_check = if msg.term == self.current_term {
                self.voted_for == 0 || self.voted_for == msg.candidate_id
            } else {
                println!("[{} {}]: -> {} Term: {}!", self.id, self.state, State::FOLLOWER, msg.term);
                self.state = State::FOLLOWER;
                self.current_term = msg.term;
                self.reset_heartbeat();
                ctx.address().do_send(FollowerHeartbeatElapseLoop {});
                true
            };

            let log_check = true;
            let vote_granted = term_check && log_check;

            if vote_granted {
                self.voted_for = msg.candidate_id;
            }

            msg.address.do_send(Voted {
                id: self.id,
                term: self.current_term,
                vote_granted,
            })
        }
    }
}

impl Handler<Voted> for Node {
    type Result = ();

    fn handle(&mut self, msg: Voted, ctx: &mut Context<Self>) -> Self::Result {
        println!("[{} {}]: Voted {}, term {}/{}, vote_granted: {}!", self.id, self.state, msg.id, msg.term, self.current_term, msg.vote_granted);
        if msg.term > self.current_term {
            println!("[{} {}]: -> {} Term: {}!", self.id, self.state, State::FOLLOWER, msg.term);
            self.state = State::FOLLOWER;
            self.current_term = msg.term;
        } else {
            for pear in &mut self.peers {
                if pear.id == msg.id {
                    if msg.vote_granted {
                        pear.is_vote_granted = true
                    } else {
                        pear.is_vote_revoked = true
                    }
                }
            }
        }

        if self.state == State::CANDIDATE  {
            let quorum = 2;
            let mut vote_granted_count = 0;
            let mut vote_revoked_count = 0;
            for pear in &mut self.peers {
                if pear.is_vote_granted {
                    vote_granted_count += 1;
                }
                if pear.is_vote_revoked {
                    vote_revoked_count += 1;
                }
            }
            if vote_granted_count >= quorum {
                self.state = State::LEADER;
                self.reset_heartbeat();
                println!("[{} {}]: Won the election with {} votes granted!", self.id, self.state, vote_granted_count);
                for pear in &mut self.peers {
                    pear.address.do_send(Append { id: self.id, term: self.current_term, ops: vec![] })
                }
                ctx.address().do_send(LeaderHeartbeatSendLoop {})
            }
            if vote_revoked_count >= quorum {
                self.state = State::FOLLOWER;
                self.reset_heartbeat();
                ctx.address().do_send(FollowerHeartbeatElapseLoop {});
                println!("[{} {}]: Lose the election with {} votes revoked!", self.id, self.state, vote_revoked_count);
            }
        }
    }
}

impl Handler<Append> for Node {
    type Result = ();

    fn handle(&mut self, msg: Append, _: &mut Context<Self>) -> Self::Result {
        self.reset_heartbeat();
    }
}