use crate::*;
use crate::mock::{*, PalletTimestamp};
use frame_support::{
    assert_ok, 
    traits::{OnFinalize, OnInitialize},
};
use pallet_subgame_assets as SubGameAssets;

/// SGB token decimals
// pub const SGB_DECIMALS: u64 = 10_000_000_000;
/// TSP token decimals
pub const TSP_DECIMALS: u64 = 1_000_000;

fn run_to_block(n: u64, t: u64) {
	while System::block_number() < n {
		TSPWhitelist::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number()+1);
        PalletTimestamp::set_timestamp(t);
		System::on_initialize(System::block_number());
		TSPWhitelist::on_initialize(System::block_number());
	}
}

fn init_asset() {
    let user = 1;
    let asset_id = 1001;
    let max_zombies = 10;
    let min_balance = 1;
    let name = "TSP".as_bytes().to_vec();
    let symbol = "TSP".as_bytes().to_vec();
    let decimals = 6;
    let mint_balance = 100000000 * TSP_DECIMALS;
    assert_ok!(SubGameAssets::Module::<Test>::_force_create(asset_id, user.clone(), max_zombies, min_balance));
    assert_ok!(SubGameAssets::Module::<Test>::_force_set_metadata(user.clone(), asset_id, name, symbol, decimals));
    assert_ok!(SubGameAssets::Module::<Test>::_mint(user.clone(), asset_id, user.clone(), mint_balance));
	
	// transfer to test user
	let to_user = 12500082579580134024;
	let amount = 100000 * TSP_DECIMALS;
    assert_ok!(SubGameAssets::Module::<Test>::_transfer(user.clone(), asset_id, to_user.clone(), amount));
}

#[test]
fn whitelist() {
    new_test_ext().execute_with(|| {
        let now: u64 = chrono::Utc::now().timestamp().saturated_into::<u64>() * 1000u64;
        run_to_block(10, now);

		init_asset();

		// let user = 1;
		// let amount = (1.1f64 * SGB_DECIMALS as f64) as u64;
        // assert_noop!(TSPWhitelist::whitelist(Origin::signed(user.clone()), amount), Error::<Test>::BuyTooLittle);

		// let user = 1;
		// let amount = 0u64 * SGB_DECIMALS;
        // assert_noop!(TSPWhitelist::whitelist(Origin::signed(user.clone()), amount), Error::<Test>::BuyTooLittle);

		// let user = 1;
		// let amount = 10000000u64 * SGB_DECIMALS;
        // assert_noop!(TSPWhitelist::whitelist(Origin::signed(user.clone()), amount), Error::<Test>::NotEnoughBalance);

		// let user = 1;
		// let amount = 300u64 * SGB_DECIMALS;
        // assert_noop!(TSPWhitelist::whitelist(Origin::signed(user.clone()), amount), Error::<Test>::BuyTooMuch);

		// let user = 1;
		// let amount = 100u64 * SGB_DECIMALS;
        // assert_noop!(TSPWhitelist::whitelist(Origin::signed(user.clone()), amount), Error::<Test>::NotWhitelist);

		// let user = 12500082579580134024;
		// let amount = 200 * SGB_DECIMALS;
        // assert_ok!(TSPWhitelist::whitelist(Origin::signed(user.clone()), amount));

		// let user = 12500082579580134024;
		// let amount = 50 * SGB_DECIMALS;
        // assert_ok!(TSPWhitelist::whitelist(Origin::signed(user.clone()), amount));

		// let user = 12500082579580134024;
		// let amount = 50 * SGB_DECIMALS;
		// assert_noop!(TSPWhitelist::whitelist(Origin::signed(user.clone()), amount), Error::<Test>::AlradyWhitelist);
    });
}

#[test]
fn add_whitelist() {
    new_test_ext().execute_with(|| {
        let now: u64 = chrono::Utc::now().timestamp().saturated_into::<u64>() * 1000u64;
        run_to_block(10, now);

        let owner = 1;
        let new_whitelist = 2;
        assert_ok!(TSPWhitelist::add_whitelist(Origin::signed(owner), new_whitelist));

        assert_eq!(TSPWhitelist::whitelist_account().len(), 13);
    });
}

#[test]
fn del_whitelist() {
    new_test_ext().execute_with(|| {
        let now: u64 = chrono::Utc::now().timestamp().saturated_into::<u64>() * 1000u64;
        run_to_block(10, now);

        let owner = 1;
        let new_whitelist = 2;
        assert_ok!(TSPWhitelist::add_whitelist(Origin::signed(owner), new_whitelist));

        let owner = 1;
        let del_whitelist = 2;
        assert_ok!(TSPWhitelist::del_whitelist(Origin::signed(owner), del_whitelist));

        assert_eq!(TSPWhitelist::whitelist_account().len(), 12);
    });
}