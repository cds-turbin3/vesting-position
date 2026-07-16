//! Step 1 smoke test: the source-free harness ingests vesting's IDL and loads
//! the program (plus mpl-core) from committed `.so` bytes. Proves the migration
//! scaffold works end to end before any test logic is ported.
#![allow(unexpected_cfgs)]

use anchor_lang::prelude::Pubkey;
use anchor_lang::{self};
use anchor_litesvm::AnchorLiteSVM;

anchor_lang::declare_program!(vesting_positions);
anchor_litesvm::bundles_from_idl!(vesting_positions);

const MPL_CORE_ID: Pubkey = Pubkey::from_str_const("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

#[test]
fn harness_ingests_idl_and_loads_the_program() {
    // Deploy both programs from committed bytes; vesting_positions (anchor 0.31)
    // is driven purely through its IDL, never compiled into this graph.
    let ctx = AnchorLiteSVM::build_with_programs(&[
        (
            vesting_positions::ID,
            "vesting_positions",
            include_bytes!("fixtures/vesting_positions.so"),
        ),
        (MPL_CORE_ID, "mpl_core", include_bytes!("fixtures/mpl_core.so")),
    ]);

    // Both programs are on-chain and executable.
    assert!(
        ctx.svm
            .get_account(&vesting_positions::ID)
            .is_some_and(|a| a.executable),
        "vesting_positions program did not load"
    );
    assert!(
        ctx.svm
            .get_account(&MPL_CORE_ID)
            .is_some_and(|a| a.executable),
        "mpl_core program did not load"
    );

    // The generated client compiled: naming a bundle forces bundles_from_idl!
    // codegen (initialize is one of vesting's 10 instructions).
    let _ = InitializeBundle::default();
}
