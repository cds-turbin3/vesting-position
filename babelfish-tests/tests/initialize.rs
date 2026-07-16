//! Vertical slice: drive `vesting_positions` initialize source-free through
//! frood, using the `frood gen` typed builders. Proves the same committed .so
//! that litesvm-tests runs on anchor-litesvm also runs on frood.

use frood::Story;
use frood_idl::types::Value;
use solana_pubkey::Pubkey;
use vesting_babelfish_tests::vesting_gen::{initialize, InitializeArgs};

const SO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/vesting_positions.so");
const MPL_CORE_SO: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/mpl_core.so");
const IDL: &str = include_str!("../idls/vesting_positions.codama.json");

const MPL_CORE_ID: Pubkey = Pubkey::from_str_const("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");

#[test]
fn initialize_creates_campaign() {
    let mut story = Story::load(SO, IDL);
    // mpl-core is not a litesvm builtin; load its .so alongside (Story.svm is pub).
    story
        .svm
        .add_program_from_file(MPL_CORE_ID, MPL_CORE_SO)
        .expect("load mpl_core.so");

    story.given("a creator with an SPL mint and a funded token account");
    let creator = story.cast("Creator");
    let mint = story.mint(&creator, 6);

    let total_deposit: u64 = 10_000_000_000_000;
    story.fund(&creator, &[(mint, total_deposit)]);

    // collection is arg-seeded ([b"collection", creator, mint, merkle_root]);
    // derive it by name from the IDL's seed template with the variable seeds.
    let merkle_root = vec![7u8; 32];
    let creator_key = creator.pubkey();
    let collection = story.pda(
        "collection",
        &[creator_key.as_ref(), mint.as_ref(), merkle_root.as_slice()],
    );
    story.alias(collection, "Collection");

    let now: i64 = 1_700_000_000;
    let start = now + 86_400;
    let out = story.when(
        "initialize is called with the default schedule",
        initialize()
            .creator(&creator)
            .mint(mint)
            .collection(collection)
            .args(InitializeArgs {
                merkle_root: merkle_root.clone(),
                start,
                end: start + 86_400 * 30,
                cliff_duration: 86_400,
                cliff_release_bps: 1_000,
                mint_to_distribute: mint,
                is_transferable: true,
                grace_period: 604_800,
                total_deposit,
                name: "Vesting campaign".to_string(),
                uri: "https://example.com/collection.json".to_string(),
            }),
        &[&creator],
    );
    assert!(out.success, "initialize failed:\n{}", out.logs.join("\n"));

    let campaign = story.pda("campaign", &[collection.as_ref()]);
    let (acct_name, val) = story.account(&campaign);
    assert_eq!(acct_name, "campaign");
    let fields = match val {
        Value::Struct(f) => f,
        other => panic!("expected Struct, got {other:?}"),
    };
    let get = |name: &str| -> Value {
        fields
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| panic!("field {name} missing"))
    };

    assert_eq!(
        get("totalDeposit"),
        Value::U64(total_deposit),
        "totalDeposit"
    );
    assert_eq!(
        get("mintToDistribute"),
        Value::Pubkey(mint.to_bytes()),
        "mintToDistribute"
    );
    assert_eq!(
        get("collection"),
        Value::Pubkey(collection.to_bytes()),
        "collection"
    );
}
