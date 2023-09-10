use crate::generic_types::Peer;
/// Generates (n) number of participiants.
/// Status - status of a peer.
/// Peer - a peer that has some status associated with it.
pub fn generate_participiants<P: Clone + Peer<Status>, Status>(n: u32) -> Vec<P> {
    (0..n)
        .collect::<Vec<u32>>()
        .iter()
        .map(|x| P::create(*x))
        .collect()
}
