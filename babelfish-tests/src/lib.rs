//! Source-free frood harness for the `vesting_positions` program.
//!
//! The same committed `.so` that `litesvm-tests/` drives through anchor-litesvm
//! is driven here through frood's `Story`, over the program's Codama IDL
//! (converted from its Anchor IDL). `vesting_gen` is the `frood gen` typed
//! mirror: a builder per instruction, so tests read close to the anchor-litesvm
//! typed form while lowering to frood's dynamic core underneath.

pub mod vesting_gen;
