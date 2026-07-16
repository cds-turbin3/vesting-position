//! Step 3 vertical slice: build the world and run `initialize` end to end on the
//! source-free harness, proving the pattern (generated bundle + account fetch +
//! mpl-core Attributes decode) before the eight tests move over in step 4.

use vesting_litesvm_tests::asset::{
    get_attr_i64, get_attr_pubkey, COL_ATTR_END, COL_ATTR_MINT, COL_ATTR_START,
};
use vesting_litesvm_tests::campaign::{CampaignConfig, TestCampaign};
use vesting_litesvm_tests::merkle::default_merkle;

#[test]
fn initialize_creates_campaign_and_stores_the_schedule() {
    let tree = default_merkle();
    let config = CampaignConfig::default();
    let world = TestCampaign::initialized(&tree, config);

    // The Campaign account exists and carries the config's schedule.
    let campaign = world.campaign();
    assert_eq!(campaign.merkle_root, tree.root);
    assert_eq!(campaign.total_deposit, config.total_deposit);
    assert_eq!(campaign.mint_to_distribute, world.mint);

    // The collection carries the schedule as mpl-core Attributes, decoded via
    // mpl-core 0.12.1's fetch_plugin and read with the ported get_attr_*.
    let attrs = world.fetch_collection_attributes();
    assert_eq!(get_attr_pubkey(&attrs, COL_ATTR_MINT), world.mint);
    assert_eq!(get_attr_i64(&attrs, COL_ATTR_START), config.start);
    assert_eq!(get_attr_i64(&attrs, COL_ATTR_END), config.end);
}
