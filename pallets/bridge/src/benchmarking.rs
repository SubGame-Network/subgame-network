use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use sp_std::{vec, vec::Vec, boxed::Box};
use sp_runtime::{
	traits::{
		SaturatedConversion, 
	}
};

#[allow(unused)]
use crate::Module as Pallet;

fn init<T: Config>() {
	let default_balances: BalanceOf<T> = 1000000u64.saturated_into();
	let owner: T::AccountId = T::OwnerAddress::get();
	<T as Config>::Balances::make_free_balance_be(&owner, default_balances);
	let user: T::AccountId = whitelisted_caller();
	<T as Config>::Balances::make_free_balance_be(&user, default_balances);
}

benchmarks! {
	send {
		init::<T>();
		let owner: T::AccountId = T::OwnerAddress::get();
		let to_address: T::AccountId = whitelisted_caller();
		let amount: BalanceOf<T> = 10u64.saturated_into();
		let coin_type: u8 = 1;
		let hash: Vec<u8> = "0x123456".clone().as_bytes().to_vec();
	}: _(RawOrigin::Signed(owner), to_address, amount, coin_type, hash)
	verify {
		
	}

	receive_bridge {
		init::<T>();
		let user: T::AccountId = whitelisted_caller();
		let to_address: Vec<u8> = "0x123456".clone().as_bytes().to_vec();
		let amount: BalanceOf<T> = 10u64.saturated_into();
		let chain_type: u8 = 2;
		let coin_type: u8 = 1;
	}: _(RawOrigin::Signed(user), to_address, amount, chain_type, coin_type)
	verify {
		
	}

	update_min_limit {
		init::<T>();
		let owner: T::AccountId = T::OwnerAddress::get();
		let amount: BalanceOf<T> = 10u64.saturated_into();
	}: _(RawOrigin::Signed(owner), amount)
	verify {
		
	}
}
