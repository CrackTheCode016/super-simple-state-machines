/// A generic, finite state machine implementation.
/// - P = Participant type
/// - S = State to keep track of and update
/// - E = Custom error type
/// TODO: Notice the use of the generic trait -= definitely need to use associated types here!
pub trait FiniteStateMachine<P: Peer<Status>, Status, S, E> {
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
pub trait Peer<Status> {
    /// Create a new peer instance.
    fn create(id: u32) -> Self;

    /// Get the current status of a peer.
    fn status(&self) -> &Status;

    /// Change the status of the peer.
    fn change_status(&mut self, status: Status);
}
