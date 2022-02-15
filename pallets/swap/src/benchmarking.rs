use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use sp_std::{vec, vec::Vec, boxed::Box};
use sp_runtime::{
	traits::{
		SaturatedConversion, 
	}
};
use frame_support::{
	assert_ok,
};

#[allow(unused)]
use crate::Module as Pallet;

pub const SGB_DECIMALS: u64 = 10_000_000_000;
pub const USDT_DECIMALS: u64 = 1_000_000;
pub const GOGO_DECIMALS: u64 = 1_000_000;

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

fn init<T: Config>() {
	let default_balances: BalanceOf<T> = 10000000000000000u64.saturated_into();
	let user: T::AccountId = whitelisted_caller();
	<T as Config>::Currency::make_free_balance_be(&user, default_balances);

    let asset_id: u32 = 7;
    let max_zombies = 10;
    let min_balance: u32 = 1;
    let name = "USDT".as_bytes().to_vec();
    let symbol = "USDT".as_bytes().to_vec();
    let decimals = 6;
    let mint_balance = 100000000 * USDT_DECIMALS;
    assert_ok!(SubGameAssets::Module::<T>::_force_create(asset_id.saturated_into(), user.clone(), max_zombies, min_balance.saturated_into()));
    assert_ok!(SubGameAssets::Module::<T>::_force_set_metadata(user.clone(), asset_id.saturated_into(), name, symbol, decimals));
    assert_ok!(SubGameAssets::Module::<T>::_mint(user.clone(), asset_id.saturated_into(), user.clone(), mint_balance.saturated_into()));

    let asset_id: u32 = 8;
    let max_zombies = 10;
    let min_balance: u32 = 1;
    let name = "GOGO".as_bytes().to_vec();
    let symbol = "GOGO".as_bytes().to_vec();
    let decimals = 6;
    let mint_balance = 100000000 * GOGO_DECIMALS;
    assert_ok!(SubGameAssets::Module::<T>::_force_create(asset_id.saturated_into(), user.clone(), max_zombies, min_balance.saturated_into()));
    assert_ok!(SubGameAssets::Module::<T>::_force_set_metadata(user.clone(), asset_id.saturated_into(), name, symbol, decimals));
    assert_ok!(SubGameAssets::Module::<T>::_mint(user.clone(), asset_id.saturated_into(), user.clone(), mint_balance.saturated_into()));
}

benchmarks! {
	create_pool {
		init::<T>();
		let user: T::AccountId = whitelisted_caller();
        let asset_x: u32 = 0;
        let x: u64 = 11 * SGB_DECIMALS;
        let asset_y: u32 = 7;
        let y: u64 = 1 * USDT_DECIMALS;
	}: _(RawOrigin::Signed(user), asset_x.saturated_into(), x.saturated_into(), asset_y.saturated_into(), y.saturated_into())
	verify {
		
	}

	add_liquidity {
		init::<T>();
		let user: T::AccountId = whitelisted_caller();

        let asset_x: u32 = 8;
        let x: u64 = 1 * GOGO_DECIMALS;
        let asset_y: u32 = 0;
        let y: u64 = 11 * SGB_DECIMALS;
        assert_ok!(Pallet::<T>::create_pool(RawOrigin::Signed(user.clone()).into(), asset_x.saturated_into(), x.saturated_into(), asset_y.saturated_into(), y.saturated_into()));

		let swap_id: u32 = 1;
        let dx: u64 = 2 * GOGO_DECIMALS;
        let dy: u64 = 22 * SGB_DECIMALS;
	}: _(RawOrigin::Signed(user), swap_id.saturated_into(), dx.saturated_into(), dy.saturated_into())
	verify {
		
	}

	remove_liquidity {
		init::<T>();
		let user: T::AccountId = whitelisted_caller();

		let asset_x: u32 = 0;
        let x: u64 = 1 * SGB_DECIMALS;
        let asset_y: u32 = 7;
        let y: u64 = 11 * USDT_DECIMALS; 
        assert_ok!(Pallet::<T>::create_pool(RawOrigin::Signed(user.clone()).into(), asset_x.saturated_into(), x.saturated_into(), asset_y.saturated_into(), y.saturated_into()));

		let swap_id: u32 = 1;
        let lp_balance: u64 = 1;
	}: _(RawOrigin::Signed(user), swap_id.saturated_into(), lp_balance.saturated_into())
	verify {
		
	}

	swap {
		init::<T>();
		let user: T::AccountId = whitelisted_caller();
		
		let asset_x: u32 = 8;
        let x: u64 = 1 * GOGO_DECIMALS;
        let asset_y: u32 = 7;
        let y: u64 = 11 * USDT_DECIMALS; 
		assert_ok!(Pallet::<T>::create_pool(RawOrigin::Signed(user.clone()).into(), asset_x.saturated_into(), x.saturated_into(), asset_y.saturated_into(), y.saturated_into()));

		let swap_id: u32 = 1;
        let input_asset: u32 = 8;
        let input_amount: u64 = 1 * GOGO_DECIMALS;
        let output_asset: u32 = 7;
        let expected_output_amount: u64 = 5 * USDT_DECIMALS; 
        let slipage: u64 = 990;
        let deadline: u64 = 30;
	}: _(RawOrigin::Signed(user), swap_id.saturated_into(), input_asset.saturated_into(), input_amount.saturated_into(), output_asset.saturated_into(), expected_output_amount.saturated_into(), slipage.saturated_into(), deadline.saturated_into())
	verify {
		
	}
}

impl_benchmark_test_suite!(
	Pallet,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
