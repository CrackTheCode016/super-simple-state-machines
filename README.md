# Super Simple State Machines

Learn Rust with super simple state machines.  These are meant to showcase various design patters by building simple state machines.

## Concepts

The primary goal of this is to showcase the use of generics and associated types in a practical environment.  Some of these patterns may seem foreign, but are akin to some concepts found within the Substrate codebase.

### Parallels to Substrate/FRAME Development

When developing within the context of a runtime environment in Substrate or FRAME, it is very often one would expect to do the following:

1. Define context-specific errors via an enum.
2. Evolving systems that invovle multi-trait designs.
3. Implementing generic interfaces is common, i.e., pallets in a runtime
4. Interaction with associated types as a means for context-specific typing (a `Peer` having a `Status` type accessible to any "parent" trait).
5. Emphasis on proper error handling (aka, handling  `Option` and `Result` as needed).

This project is intentionally abstract, which better prepares for the development paradigms that Susbtrate usually adopts.

### Traits

- **Peer** - The "peers" who are participating in the system.  Peers get rewarded for changing the state, but only if they are an `Author`.  If they aren't - they get put in `Timeout` and cannot author new changes. Each peer has a `Status`, which is generic.

- **FiniteStateMachine** - THe base trait for how our state machine may operate.  The state is abstracted away. This allows for mutliple state machines to be created with the same expectations, which could also lead to them communicating with one another.  It accepts three generic parameters to denote the peer, state, and error.

## Machines

- Boolean status machine (`machine_simple_impl.rs`) - simply tracks status as on (`true`) or off (`false`). Picks the next participant in the peer list, and keeps cycling forever.

## Running & Testing

For running a simple state machine example:

```sh
cargo run
```

Which should yield the following output:

For running tests:

```sh
# For all test
cargo test

# For a specific test
cargo test machine_simple_impl
```
