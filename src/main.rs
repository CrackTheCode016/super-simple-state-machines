use std::{collections::HashMap, time::Duration};
mod machine_simple_impl;
mod generic_types;

use generic_types::FiniteStateMachine;
use machine_simple_impl::{Participant, ParticipantStatus, MachineError, Machine, State};

/// The super simple state machine that:
///
///     1. Propels from one state to another, while keeping the previous state in memory.
///     2. Determines if the person proposing this change is valid
///         2a. A ("random") vote for who should be a leader.
///         2b. Once selected, the machine should be aware of the leader.
///         2c. Every "era", the leader will die and go on cooldown.  
///         2d. If a follower attempts to change state, they get a timeout.

fn main() -> Result<(), MachineError> {
    let p1 = Participant {
        address: 0,
        status: ParticipantStatus::Participant,
    };
    let p2 = Participant {
        address: 1,
        status: ParticipantStatus::Participant,
    };

    let p3: Participant = Participant {
        address: 2,
        status: ParticipantStatus::Participant,
    };
    let p4: Participant = Participant {
        address: 3,
        status: ParticipantStatus::Participant,
    };
    let author: Participant = Participant {
        address: 4,
        status: ParticipantStatus::Author,
    };

    let mut peers = HashMap::new();

    peers.insert(p1.address, p1.clone());
    peers.insert(p2.address, p2);
    peers.insert(p3.address, p3.clone());
    peers.insert(p4.address, p4);
    peers.insert(author.address, author.clone());

    let intital_state = Machine::<State>::inital();
    let mut machine = Machine::<State>::new(intital_state, peers, author.clone());

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
