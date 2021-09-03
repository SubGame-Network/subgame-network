use crate::{Error, mock::*};
use frame_support::{assert_noop, assert_ok};
use pallet_subgame_assets as SubGameAssets;

pub const SGB_DECIMALS: u64 = 10_000_000_000;
pub const USDT_DECIMALS: u64 = 1_000_000;
pub const GOGO_DECIMALS: u64 = 1_000_000;
pub const LP_DECIMALS: u64 = 1_000_000;

fn init_asset() {
    let user = 1;
    let asset_id = 7;
    let max_zombies = 10;
    let min_balance = 1;
    let name = "USDT".as_bytes().to_vec();
    let symbol = "USDT".as_bytes().to_vec();
    let decimals = 6;
    let mint_balance = 1000000 * USDT_DECIMALS;
    assert_ok!(SubGameAssets::Module::<Test>::_force_create(asset_id, user.clone(), max_zombies, min_balance));
    assert_ok!(SubGameAssets::Module::<Test>::_force_set_metadata(user.clone(), asset_id, name, symbol, decimals));
    assert_ok!(SubGameAssets::Module::<Test>::_mint(user.clone(), asset_id, user.clone(), mint_balance));

    let user = 1;
    let asset_id = 8;
    let max_zombies = 10;
    let min_balance = 1;
    let name = "GOGO".as_bytes().to_vec();
    let symbol = "GOGO".as_bytes().to_vec();
    let decimals = 6;
    let mint_balance = 1000000 * GOGO_DECIMALS;
    assert_ok!(SubGameAssets::Module::<Test>::_force_create(asset_id, user.clone(), max_zombies, min_balance));
    assert_ok!(SubGameAssets::Module::<Test>::_force_set_metadata(user.clone(), asset_id, name, symbol, decimals));
    assert_ok!(SubGameAssets::Module::<Test>::_mint(user.clone(), asset_id, user.clone(), mint_balance));
}

#[test]
fn create_pool() {
    new_test_ext().execute_with(|| {
        init_asset();

        let user = 1;
        let asset_x: u32 = 0;
        let x: u64 = 11 * SGB_DECIMALS;
        let asset_y: u32 = 7;
        let y: u64 = 1 * USDT_DECIMALS;
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));

        let swap_pool = Swap::swap_pool(1);
        println!("===\n{:?}\n===", swap_pool);
        assert_eq!(asset_x, swap_pool.asset_x);
        assert_eq!(asset_y, swap_pool.asset_y);

        // Check pool asset a balance
        let got_x = Balances::free_balance(&swap_pool.account);
        assert_eq!(x, got_x);

        // Check pool asset b balance
        let got_y = SubGameAssets::Module::<Test>::balance(asset_y, swap_pool.account);
        assert_eq!(y, got_y);

        // Check LP token name
        let got_lp_token_name = SubGameAssets::Module::<Test>::_get_metadata(swap_pool.asset_lp).name;
        assert_eq!("SGB-USDT LP", core::str::from_utf8(&got_lp_token_name).unwrap());

        // Check pool should return exists error
        let user = 1;
        let asset_x: u32 = 7;
        let x: u64 = 1 * USDT_DECIMALS;
        let asset_y: u32 = 0;
        let y: u64 = 11 * SGB_DECIMALS;
        assert_noop!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y), Error::<Test>::SwapAlreadyExists);
    });
}

#[test]
fn create_pool2() {
    new_test_ext().execute_with(|| {
        init_asset();

        // Should return not enough balance error
        let user = 1;
        let asset_x: u32 = 8;
        let x: u64 = 1000000 * GOGO_DECIMALS + 100;
        let asset_y: u32 = 0;
        let y: u64 = 11 * SGB_DECIMALS;
        assert_noop!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y), Error::<Test>::NotEnoughBalance);

        let user = 1;
        let asset_x: u32 = 8;
        let x: u64 = 1 * GOGO_DECIMALS;
        let asset_y: u32 = 0;
        let y: u64 = 11 * SGB_DECIMALS;
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));

        let swap_pool = Swap::swap_pool(1);
        println!("===\n{:?}\n===", swap_pool);
        assert_eq!(asset_x, swap_pool.asset_x);
        assert_eq!(asset_y, swap_pool.asset_y);

        let got_y = Balances::free_balance(&swap_pool.account);
        assert_eq!(y, got_y);

        let got_x = SubGameAssets::Module::<Test>::balance(asset_x, swap_pool.account);
        assert_eq!(x, got_x);

        let got_lp_token_name = SubGameAssets::Module::<Test>::_get_metadata(swap_pool.asset_lp).name;
        assert_eq!("GOGO-SGB LP", core::str::from_utf8(&got_lp_token_name).unwrap());
    });
}

#[test]
fn add_liquidity() {
    new_test_ext().execute_with(|| {
        init_asset();

        let user = 1;
        let asset_x: u32 = 8;
        let x: u64 = 1 * GOGO_DECIMALS;
        let asset_y: u32 = 0;
        let y: u64 = 11 * SGB_DECIMALS;
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));
        let swap_pool = Swap::swap_pool(1);

        // Should return zero balance error
        let user = 1;
        let swap_id = 1;
        let dx: u64 = 0 * GOGO_DECIMALS;
        let dy: u64 = 22 * SGB_DECIMALS;
        assert_noop!(Swap::add_liquidity(Origin::signed(user.clone()), swap_id, dx, dy), Error::<Test>::ZeroBalance);

        // Should return liquidity error
        let user = 1;
        let swap_id = 1;
        let dx: u64 = 1 * GOGO_DECIMALS;
        let dy: u64 = 22 * SGB_DECIMALS;
        assert_noop!(Swap::add_liquidity(Origin::signed(user.clone()), swap_id, dx, dy), Error::<Test>::LiquidityKError);

        let user = 1;
        let swap_id = 1;
        let dx: u64 = 2 * GOGO_DECIMALS;
        let dy: u64 = 22 * SGB_DECIMALS;
        assert_ok!(Swap::add_liquidity(Origin::signed(user.clone()), swap_id, dx, dy));

        // Check LP token balance
        let got_lp_balance = SubGameAssets::Module::<Test>::total_supply(swap_pool.asset_lp);
        let _lp_total_supply = (x + y) / 2;
        let want_lp_balance = dx / x * _lp_total_supply + _lp_total_supply;
        assert_eq!(want_lp_balance, got_lp_balance);
    });
}

#[test]
fn remove_liquidity() {
    new_test_ext().execute_with(|| {
        init_asset();

        let user = 1;
        let asset_x: u32 = 8;
        let x: u64 = 1 * GOGO_DECIMALS;
        let asset_y: u32 = 7;
        let y: u64 = 11 * USDT_DECIMALS; 
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));
        let swap_pool = Swap::swap_pool(1);

        // Should return not enough LP token error
        let user = 2;
        let swap_id = 1;
        let lp_balance: u64 = 1 * LP_DECIMALS;
        assert_noop!(Swap::remove_liquidity(Origin::signed(user.clone()), swap_id, lp_balance), Error::<Test>::NotEnoughLPToken);

        let user = 1;
        let swap_id = 1;
        let lp_balance: u64 = 6 * LP_DECIMALS;
        assert_ok!(Swap::remove_liquidity(Origin::signed(user.clone()), swap_id, lp_balance));

        // Check LP token balance
        let got_user_lp_balance: u64 = SubGameAssets::Module::<Test>::balance(swap_pool.asset_lp, user);
        let want_user_lp_balance: u64 = 0;
        assert_eq!(want_user_lp_balance, got_user_lp_balance);
    });
}

#[test]
fn swap() {
    new_test_ext().execute_with(|| {
        init_asset();

        let user = 1;
        let asset_x: u32 = 8;
        let x: u64 = 1 * GOGO_DECIMALS;
        let asset_y: u32 = 7;
        let y: u64 = 11 * USDT_DECIMALS; 
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));
        let swap_pool = Swap::swap_pool(1);

        let before_user_y_balance = SubGameAssets::Module::<Test>::balance(swap_pool.asset_y, user);

        let swap_id = 1;
        let input_asset: u32 = 8;
        let input_amount: u64 = 1 * GOGO_DECIMALS;
        let output_asset: u32 = 7;
        let expected_output_amount: u64 = 5 * USDT_DECIMALS; 
        let slipage: u64 = 15;
        let deadline: u64 = 30;
        assert_ok!(Swap::swap(Origin::signed(user.clone()), swap_id, input_asset, input_amount, output_asset, expected_output_amount, slipage, deadline));

        let after_user_y_balance = SubGameAssets::Module::<Test>::balance(swap_pool.asset_y, user);

        // Check pool balance
        assert_eq!(input_amount + x, SubGameAssets::Module::<Test>::balance(swap_pool.asset_x, swap_pool.account));
        let want_pool_y_balance = y - (after_user_y_balance - before_user_y_balance);
        assert_eq!(want_pool_y_balance, SubGameAssets::Module::<Test>::balance(swap_pool.asset_y, swap_pool.account));

        // Should return AssetNotFound error
        let _input_asset: u32 = 5;
        let _output_asset: u32 = 7;
        assert_noop!(Swap::swap(Origin::signed(user.clone()), swap_id, _input_asset, input_amount, _output_asset, expected_output_amount, slipage, deadline), Error::<Test>::AssetNotFound);
        let _input_asset: u32 = 8;
        let _output_asset: u32 = 5;
        assert_noop!(Swap::swap(Origin::signed(user.clone()), swap_id, _input_asset, input_amount, _output_asset, expected_output_amount, slipage, deadline), Error::<Test>::AssetNotFound);
        
        // Should return ZeroExpectedAmount error
        let _expected_output_amount: u64 = 0; 
        assert_noop!(Swap::swap(Origin::signed(user.clone()), swap_id, input_asset, input_amount, output_asset, _expected_output_amount, slipage, deadline), Error::<Test>::ZeroExpectedAmount);
        
        // Should return ZeroBalance error
        let _input_amount: u64 = 0 * GOGO_DECIMALS;
        assert_noop!(Swap::swap(Origin::signed(user.clone()), swap_id, input_asset, _input_amount, output_asset, expected_output_amount, slipage, deadline), Error::<Test>::ZeroBalance);
    });
}