# Super Simple State Machines

Learn Rust with super simple state machines.  These are meant to showcase various design patters by building simple state machines.  

## Concepts

- **Participants** - The "peers" who are participating in the system.  Peers get rewarded for changing the state, but only if they are an `Author`.  If they aren't - they get put in `Timeout` and cannot author new changes.
  

## Machines

- Boolean status machine (`machine_simple_impl.rs`) - simply tracks status on and off. Picks the next participant in the peer list, and keeps cycling forever.

## Running & Testing