use crate::generic_types::{FiniteStateMachine, Peer};
use std::collections::HashMap;

/// Boolean status machine - flip it on or off!

/// The type of participant
#[derive(Clone, Debug, PartialEq)]
pub enum ParticipantStatus {
    Author,
    Participant,
    Timeout,
}

/// An error specific to our machine
#[derive(Clone, Debug, PartialEq)]
pub enum MachineError {
    MaliciousParticipant(u32),
}

/// A participant that "runs" the machine
#[derive(Clone, Debug)]
pub struct Participant {
    pub address: u32,
    pub status: ParticipantStatus,
}

/// Our core machine's state - which in this case, is a simple boolean.
/// More can be added.  Feel free to make this more complex!
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    /// Status - can be on or off (true == on, false == off)
    pub status: bool,
}

/// A simple machine for state
pub struct SimpleMachine<S> {
    pub state: S,
    pub prev_state: Option<S>,
    pub current_author: Participant,
    pub peers: HashMap<u32, Participant>,
}

impl FiniteStateMachine<Participant, State, MachineError> for SimpleMachine<State> {
    fn inital() -> State {
        State { status: false }
    }

    fn print_state(&self) {
        println!("Current state of status: {}", self.state.status);
        println!("Current author {:#?}", self.current_author);
    }

    fn print_authors(&self) {
        println!("PEERS LIST: {:#?}", self.peers);
    }

    fn next(&mut self, author: &mut Participant) -> Result<State, MachineError> {
        if author.status != ParticipantStatus::Author {
            self.punish(author);
            return Err(MachineError::MaliciousParticipant(author.address));
        }
        self.prev_state = Some(self.state);
        self.state.status = !self.state.status;
        Ok(self.state)
    }

    fn revert(&mut self) {
        let prev = self.prev_state;
        if let Some(prev) = prev {
            self.state = prev;
            self.prev_state = None;
        }
    }

    fn pick_author(&mut self) {
        // Current author demoted
        match self.peers.get(&self.current_author.address) {
            Some(author) => {
                let mut author = author.clone();
                author.status = ParticipantStatus::Participant;
                self.peers.insert(author.address, author.clone());
            }
            None => todo!(),
        };

        // Pick new author, just picks the next one for now
        let proposed = self.peers.get(&(self.current_author.address + 1));
        if let Some(proposed) = proposed {
            if proposed.status != ParticipantStatus::Timeout {
                self.current_author = proposed.clone();
                self.current_author.status = ParticipantStatus::Author;
                self.peers
                    .insert(self.current_author.address, self.current_author.clone());
            }
        } else {
            self.current_author = self.peers.get(&0).expect("No 0th peer found").clone();
            if self.current_author.status != ParticipantStatus::Timeout {
                self.current_author.status = ParticipantStatus::Author;
                self.peers
                    .insert(self.current_author.address, self.current_author.clone());
            }
        }
    }

    /// For this implementation, if they are not a leader, they get put in timeout.
    fn punish(&mut self, participant: &mut Participant) {
        let mut bad_participant = self
            .peers
            .get(&participant.address)
            .expect("Peer doesn't exist")
            .clone();
        bad_participant.change_status(ParticipantStatus::Timeout);
        self.peers
            .insert(participant.address, bad_participant.clone());
    }
}

impl<S> SimpleMachine<S> {
    pub fn new(s: S, peers: HashMap<u32, Participant>, current_author: Participant) -> Self {
        SimpleMachine {
            state: s,
            prev_state: None,
            current_author,
            peers,
        }
    }
}

impl Peer for Participant {
    type Status = ParticipantStatus;
    fn create(id: u32) -> Self {
        Participant {
            address: id,
            status: ParticipantStatus::Participant,
        }
    }

    fn status(&self) -> &ParticipantStatus {
        &self.status
    }

    fn change_status(&mut self, status: ParticipantStatus) {
        self.status = status;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::generate_participiants;

    fn create_machine_with_peers() -> SimpleMachine<State> {
        let peer_gen = generate_participiants::<Participant>(3);
        let mut peers = HashMap::new();
        for peer in peer_gen {
            peers.insert(peer.address, peer.clone());
        }

        let mut author = peers.get(&0).unwrap().clone();
        author.change_status(ParticipantStatus::Author);
        peers.insert(author.address, author.clone());

        let intital_state = SimpleMachine::<State>::inital();
        SimpleMachine::<State>::new(intital_state, peers, author.clone())
    }

    #[test]
    fn machine_successfully_created() {
        let machine = create_machine_with_peers();
        assert_eq!(machine.state.status, false);
    }

    #[test]
    fn machine_changes_author() {
        let mut machine = create_machine_with_peers();
        machine.pick_author();
        let author = machine.peers.get(&1).unwrap().clone();
        assert_eq!(author.address, 1);
        assert_eq!(machine.state.status, false);
    }

    #[test]
    fn machine_advances_state() {
        let mut machine = create_machine_with_peers();
        let mut author = machine.peers.get(&0).unwrap().clone();
        assert!(machine.next(&mut author).is_ok());
        assert_eq!(machine.state.status, true);
    }

    #[test]
    fn machine_punishes() {
        let mut machine = create_machine_with_peers();
        let mut participant = machine.peers.get(&1).unwrap().clone();
        assert!(machine.next(&mut participant).is_err());
        let participant = machine.peers.get(&1).unwrap().clone();
        assert_eq!(*participant.status(), ParticipantStatus::Timeout);
    }
}
