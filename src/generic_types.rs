/// A generic, finite state machine implementation.
/// - P = Participant type
/// - S = State to keep track of and update
/// - E = Custom error type
pub trait FiniteStateMachine<P, S, E> {
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
