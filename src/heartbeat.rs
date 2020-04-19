use actix::prelude::*;
use std::time::{Duration, SystemTime};
use rand::Rng;

use super::model::{Node, State};
use super::election::{NewElectionRound, Append};

#[derive(Message)]
#[rtype(result = "()")]
pub struct FollowerHeartbeatElapseLoop {}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaderHeartbeatSendLoop {}

impl Handler<FollowerHeartbeatElapseLoop> for Node {
    type Result = ();

    fn handle(&mut self, _: FollowerHeartbeatElapseLoop, ctx: &mut Context<Self>) -> Self::Result {
        let heartbeat_interval = Duration::from_secs(10).as_millis();
        let election_start_delay = rand::thread_rng().gen_range(0, 3000);

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_millis();
        if self.state == State::LEADER {
            return;
        } else if self.state == State::FOLLOWER && self.heartbeat == 0 {
            println!("[{} {}]: -> {} Heartbeat: {}!", self.id, self.state, State::CANDIDATE, self.heartbeat);
            self.reset_election();
            ctx.run_later(Duration::from_millis(election_start_delay), |_act, ctx| {
                ctx.address().do_send(NewElectionRound {});
            });
        } else if self.state == State::FOLLOWER && (self.heartbeat + heartbeat_interval) < now {
            println!("[{} {}]: -> {} Heartbeat interval: {}!", self.id, self.state, State::CANDIDATE, now - (self.heartbeat + heartbeat_interval));
            self.reset_election();
            ctx.run_later(Duration::from_millis(election_start_delay), |_act, ctx| {
                ctx.address().do_send(NewElectionRound {});
            });
        } else {
            ctx.run_later(Duration::from_secs(3), |_act, ctx| {
                ctx.address().do_send(FollowerHeartbeatElapseLoop {})
            });
        }
        return;
    }
}

impl Handler<LeaderHeartbeatSendLoop> for Node {
    type Result = ();

    fn handle(&mut self, _: LeaderHeartbeatSendLoop, ctx: &mut Context<Self>) -> Self::Result {
        let heartbeat_interval = Duration::from_secs(5).as_millis();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap()
            .as_millis();

        if self.state == State::LEADER {
            if (self.heartbeat + heartbeat_interval) < now {
                for pear in &mut self.peers {
                    pear.address.do_send(Append { id: self.id, term: self.current_term, ops: vec![] })
                }
            }
            ctx.run_later(Duration::from_millis(500), |_act, ctx| {
                ctx.address().do_send(LeaderHeartbeatSendLoop {})
            });
        }

    }
}

impl Node {
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
