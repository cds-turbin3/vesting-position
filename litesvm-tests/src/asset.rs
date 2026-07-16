//! Collection-attribute readers the tests need but the IDL can't provide:
//! `get_attr_*` (ported from `vesting_positions::utils::asset`) read the schedule
//! the program stores on the mpl-core collection as string Attributes. They take
//! an `mpl_core::types::Attributes` (decoded from the collection account by the
//! world builder via `fetch_plugin`), so this crate depends on mpl-core 0.12.1,
//! the first release on solana 3.x. Test-crate versions panic on a missing or
//! unparseable attribute rather than returning the program's anchor `Result`.

use anchor_lang::prelude::Pubkey;
use mpl_core::types::Attributes;

/// Attribute keys the program writes onto the collection (must match
/// `vesting_positions::constants`).
pub const COL_ATTR_MINT: &str = "mint";
pub const COL_ATTR_START: &str = "start";
pub const COL_ATTR_END: &str = "end";
pub const COL_ATTR_CLIFF_DURATION: &str = "cliff_duration";
pub const COL_ATTR_CLIFF_RELEASE_BPS: &str = "cliff_release_bps";
pub const COL_ATTR_GRACE_PERIOD: &str = "grace_period";

fn value<'a>(attrs: &'a Attributes, key: &str) -> &'a str {
    &attrs
        .attribute_list
        .iter()
        .find(|a| a.key == key)
        .unwrap_or_else(|| panic!("collection attribute `{key}` missing"))
        .value
}

pub fn get_attr_u64(attrs: &Attributes, key: &str) -> u64 {
    value(attrs, key)
        .parse()
        .unwrap_or_else(|_| panic!("attribute `{key}` is not a u64"))
}

pub fn get_attr_i64(attrs: &Attributes, key: &str) -> i64 {
    value(attrs, key)
        .parse()
        .unwrap_or_else(|_| panic!("attribute `{key}` is not an i64"))
}

pub fn get_attr_pubkey(attrs: &Attributes, key: &str) -> Pubkey {
    value(attrs, key)
        .parse()
        .unwrap_or_else(|_| panic!("attribute `{key}` is not a pubkey"))
}
