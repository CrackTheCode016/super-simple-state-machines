/// A generic, finite state machine implementation.
/// - P = Participant type
/// - S = State to keep track of and update
/// - E = Custom error type
/// TODO: Notice the use of the generic trait -= definitely need to use associated types here!
/// TODO: Add some common "Config" trait for the state machine? Replaces the generic parameters?
/// TODO: It may be better to illustrate some common Config for the whole machine,
/// Or once the machine is implemented, then we have access to various types for other implementations.
pub trait FiniteStateMachine<P: Peer, S, E> {
    /// Defines some initial state
    fn inital() -> S;

    /// Prints the state
    fn print_state(&self);

    /// Prints all peers in our machine
    fn print_authors(&self);

    /// Updates to a new state
    fn next(&mut self, p: &mut P) -> Result<S, E>;

    /// Reverts to the previous state
    fn revert(&mut self);

    /// Punishes a bad actor if they attempt an adversarial action
    fn punish(&mut self, p: &mut P);

    /// Picks author
    fn pick_author(&mut self);
}

/// A generic peer for a state machine.
/// Note we could've done: pub trait Peer<Status>, aka a generic. However, in implementation
/// we would need to define what Status is each time we use this trait. i.e., P: Peer<Status>, Status
/// Hence, the associated type, which allows us to define Status for our Peer once.
/// It is also scope specific - why should a machine care about the status of the Peer: impl FiniteStateMachine<Participant, ParticipantStatus, State, MachineError> for Machine<State>
/// Associated types allow us to fully leverage generic typing, scope it, and define a DX.
pub trait Peer {
    type Status;
    /// Create a new peer instance.
    fn create(id: u32) -> Self;

    /// Get the current status of a peer.
    fn status(&self) -> &Self::Status;

    /// Change the status of the peer.
    fn change_status(&mut self, status: Self::Status);
}
