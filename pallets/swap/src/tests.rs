use crate::{Error, mock::*};
use frame_support::{assert_noop, assert_ok, traits::{OnFinalize, OnInitialize}};
use pallet_subgame_assets as SubGameAssets;

pub const SGB_DECIMALS: u64 = 10_000_000_000;
pub const USDT_DECIMALS: u64 = 1_000_000;
pub const GOGO_DECIMALS: u64 = 1_000_000;
pub const LP_DECIMALS: u64 = 1_000_000;

fn run_to_block( n: u64) {
	while System::block_number() < n {
		Swap::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number()+1);
		System::on_initialize(System::block_number());
		Swap::on_initialize(System::block_number());
	}
}

fn init_asset() {
    let user = 1;
    let asset_id = 7;
    let max_zombies = 10;
    let min_balance = 1;
    let name = "USDT".as_bytes().to_vec();
    let symbol = "USDT".as_bytes().to_vec();
    let decimals = 6;
    let mint_balance = 100000000 * USDT_DECIMALS;
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
    let mint_balance = 100000000 * GOGO_DECIMALS;
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
        let x: u64 = 100000000 * GOGO_DECIMALS + 100;
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
fn create_pool_lp_check() {
    new_test_ext().execute_with(|| {
        init_asset();

        let user = 1;
        let asset_x: u32 = 8;
        let x: u64 = 1000000000;
        let asset_y: u32 = 7;
        let y: u64 = 200000000;
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));

        let swap_pool = Swap::swap_pool(1);
        let got_lp_balance = SubGameAssets::Module::<Test>::total_supply(swap_pool.asset_lp);
        let want_lp_balance = (libm::sqrt((x as f64 / GOGO_DECIMALS as f64) * (y as f64 / USDT_DECIMALS as f64)) * LP_DECIMALS as f64).floor() as u64;
        println!("===");
        println!("got_lp_balance = {:?}, want_lp_balance = {:?}", got_lp_balance, want_lp_balance);
        println!("===");
        assert_eq!(want_lp_balance, got_lp_balance);

        run_to_block(10);

        let user = 1;
        let asset_x: u32 = 0;
        let x: u64 = 350000000000000;
        let asset_y: u32 = 7;
        let y: u64 = 35000000000;
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));

        let swap_pool = Swap::swap_pool(2);
        let got_lp_balance = SubGameAssets::Module::<Test>::total_supply(swap_pool.asset_lp);
        let want_lp_balance = (libm::sqrt((x as f64 / SGB_DECIMALS as f64) * (y as f64 / USDT_DECIMALS as f64)) * LP_DECIMALS as f64).floor() as u64;
        println!("===");
        println!("got_lp_balance = {:?}, want_lp_balance = {:?}", got_lp_balance, want_lp_balance);
        println!("===");
        assert_eq!(want_lp_balance, got_lp_balance);
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
        let swap_pool = Swap::swap_pool(1);
        let got_lp_balance = SubGameAssets::Module::<Test>::total_supply(swap_pool.asset_lp);
        let _lp_total_supply = (libm::sqrt((x as f64 / GOGO_DECIMALS as f64) * (y as f64 / SGB_DECIMALS as f64)) as f64 * LP_DECIMALS as f64).floor() as u64;
        let want_lp_balance = ((dx as f64 / x as f64) * _lp_total_supply as f64).floor() as u64 + _lp_total_supply;
        assert_eq!(want_lp_balance, got_lp_balance);
    });
}

#[test]
fn add_liquidity2() {
    new_test_ext().execute_with(|| {
        init_asset();

        let user = 1;
        let asset_x: u32 = 0;
        let x: u64 = 350000000000000;
        let asset_y: u32 = 7;
        let y: u64 = 35000000000;
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));

        let user = 1;
        let swap_id = 1;
        let dx: u64 = 50000000000000;
        let dy: u64 = 5000000000;
        assert_ok!(Swap::add_liquidity(Origin::signed(user.clone()), swap_id, dx, dy));
    });
}

#[test]
fn add_liquidity3() {
    new_test_ext().execute_with(|| {
        init_asset();

        let user = 1;
        let asset_x: u32 = 0;
        let x: u64 = 338520327881663;
        let asset_y: u32 = 7;
        let y: u64 = 25170352201;
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));

        let swap_id = 1;
        let new_lp_balance = 145856159058418;
        let swap_pool = Swap::swap_pool(swap_id);
        let old_lp_balance = SubGameAssets::Module::<Test>::total_supply(swap_pool.asset_lp);
        assert_ok!(SubGameAssets::Module::<Test>::_burn(swap_pool.account, swap_pool.asset_lp, user, old_lp_balance));
        assert_ok!(SubGameAssets::Module::<Test>::_mint(swap_pool.account, swap_pool.asset_lp, user, new_lp_balance));
        
        let user = 1;
        let dx: u64 = 13449169291;
        let dy: u64 = 1000000;
        assert_ok!(Swap::add_liquidity(Origin::signed(user.clone()), swap_id, dx, dy));
    });
}

#[test]
fn add_liquidity4() {
    new_test_ext().execute_with(|| {
        init_asset();

        let user = 1;
        let asset_x: u32 = 8;
        let x: u64 = 267148620;
        let asset_y: u32 = 0;
        let y: u64 = 29617744175575;
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));

        let swap_id = 1;
        let new_lp_balance = 925091992;
        let swap_pool = Swap::swap_pool(swap_id);
        let old_lp_balance = SubGameAssets::Module::<Test>::total_supply(swap_pool.asset_lp);
        assert_ok!(SubGameAssets::Module::<Test>::_burn(swap_pool.account, swap_pool.asset_lp, user, old_lp_balance));
        assert_ok!(SubGameAssets::Module::<Test>::_mint(swap_pool.account, swap_pool.asset_lp, user, new_lp_balance));
        
        let user = 1;
        let dx: u64 = 1000000;
        let dy: u64 = 110866153481;
        assert_ok!(Swap::add_liquidity(Origin::signed(user.clone()), swap_id, dx, dy));
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

        // Should return not enough LP token error
        let user = 2;
        let swap_id = 1;
        let lp_balance: u64 = 7;
        assert_noop!(Swap::remove_liquidity(Origin::signed(user.clone()), swap_id, lp_balance), Error::<Test>::NotEnoughLPToken);
    });
}

#[test]
fn remove_liquidity_too_many() {
    new_test_ext().execute_with(|| {
        init_asset();

        let user = 1;
        let asset_x: u32 = 0;
        let x: u64 = 1 * SGB_DECIMALS;
        let asset_y: u32 = 7;
        let y: u64 = 11 * USDT_DECIMALS; 
        assert_ok!(Swap::create_pool(Origin::signed(user.clone()), asset_x, x, asset_y, y));

        let user = 1;
        let swap_id = 1;
        let lp_balance: u64 = 3316624;
        assert_noop!(Swap::remove_liquidity(Origin::signed(user.clone()), swap_id, lp_balance), Error::<Test>::TooManyLPToken);
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

        // Should return Slipage error
        let swap_id = 1;
        let input_asset: u32 = 8;
        let input_amount: u64 = 1 * GOGO_DECIMALS;
        let output_asset: u32 = 7;
        let expected_output_amount: u64 = 5 * USDT_DECIMALS; 
        let slipage: u64 = 90;
        let deadline: u64 = 30;
        assert_noop!(Swap::swap(Origin::signed(user.clone()), swap_id, input_asset, input_amount, output_asset, expected_output_amount, slipage, deadline), Error::<Test>::Slipage);

        let swap_id = 1;
        let input_asset: u32 = 8;
        let input_amount: u64 = 1 * GOGO_DECIMALS;
        let output_asset: u32 = 7;
        let expected_output_amount: u64 = 5 * USDT_DECIMALS; 
        let slipage: u64 = 990;
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

#[test]
fn swap2() {
    new_test_ext().execute_with(|| {

        // $r = 1 - 0.003
		// $a = $dx / $x
		// $dy = ($a * $r) / (1 + ($a * $r)) * $y
        let x: f64 = 9378908395443.0;
        let y: f64 = 1037063538.0;
        let dx: f64 = 10000000000.25;
        let r: f64 = 1.0 - 0.003;
        let a: f64 = dx / x;
        let dy: f64 = (a * r) / (1.0 + (a * r)) * y;
        let output_amount: u64 = dy.floor() as u64;

        println!("{:?}", output_amount);
    });
}