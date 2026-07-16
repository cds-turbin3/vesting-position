//! Schedule math mirrored from the program's `utils/vesting.rs`, so tests can
//! compute an expected release independently of the on-chain result and assert
//! the two agree. It runs over the generated `Campaign` (same fields as the
//! program's state), and returns `None` on arithmetic overflow rather than the
//! program's `Result`, since a test has no `ErrorCode` to map onto.
//!
//! The program keeps its own unit tests for the overflow edges; here the
//! integration tests exercise this against real claim balances, which is a
//! stronger check than a hand-built `Campaign` in isolation.

use crate::vesting_positions::accounts::Campaign;

/// Tokens claimable at `now` for a position of `allocation`, net of
/// `claimed_so_far`. `None` on overflow.
pub fn compute_claimable(
    campaign: &Campaign,
    now: i64,
    allocation: u64,
    claimed_so_far: u64,
) -> Option<u64> {
    let cliff_end = campaign.start.checked_add(campaign.cliff_duration as i64)?;
    if now < cliff_end {
        return Some(0);
    }

    // Clamp to the campaign end so post-end claims vest exactly 100%.
    let now = now.min(campaign.end);

    // u128 intermediates: allocation * bps and linear * elapsed can overflow u64
    // for large allocations over long vesting windows.
    let cliff_amount =
        ((allocation as u128).checked_mul(campaign.cliff_release_bps as u128)? / 10_000) as u64;

    let linear_amount = allocation.saturating_sub(cliff_amount);
    let vesting_window = (campaign.end - cliff_end) as u64;

    let linear_vested = if vesting_window == 0 {
        linear_amount
    } else {
        ((linear_amount as u128).checked_mul((now - cliff_end) as u128)? / vesting_window as u128)
            as u64
    };

    let total_vested = cliff_amount.checked_add(linear_vested)?.min(allocation);
    Some(total_vested.saturating_sub(claimed_so_far))
}
