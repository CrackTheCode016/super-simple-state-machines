use std::{collections::HashMap, time::Duration};
mod generic_types;
mod machine_simple_impl;
mod util;

use generic_types::{FiniteStateMachine, Peer};
use machine_simple_impl::{MachineError, Participant, ParticipantStatus, SimpleMachine, State};
use util::generate_participiants;

/// The super simple state machine that:
///
///     1. Propels from one state to another, while keeping the previous state in memory.
///     2. Determines if the person proposing this change is valid
///         2a. A ("random") vote for who should be a leader.
///         2b. Once selected, the machine should be aware of the leader.
///         2c. Every "era", the leader will die and go on cooldown.
///         2d. If a follower attempts to change state, they get a timeout.

fn main() -> Result<(), MachineError> {
    let peer_gen = generate_participiants::<Participant>(3);
    let mut peers = HashMap::new();
    for peer in peer_gen {
        peers.insert(peer.address, peer.clone());
    }

    let mut author = peers.get(&0).unwrap().clone();
    author.change_status(ParticipantStatus::Author);

    let intital_state = SimpleMachine::<State>::inital();
    let mut machine = SimpleMachine::<State>::new(intital_state, peers, author.clone());

    loop {
        let mut current_author = machine.current_author.clone();
        machine
            .next(&mut current_author)
            .unwrap_or_else(|_| machine.state);
        machine.print_state();
        machine.print_authors();
        machine.pick_author();
        std::thread::sleep(Duration::from_secs(2));
    }
}
