//! Source-free litesvm test harness for the `vesting_positions` program.
//!
//! The program (anchor 0.31 / solana 2.x) is driven through its committed `.so`
//! + IDL, so this crate stays on the modern anchor-litesvm stack without the
//! program ever entering its dependency graph. Shared helpers (merkle, campaign
//! world-builder) land here as the migration proceeds; for now it is the lib
//! target the integration tests compile against.
