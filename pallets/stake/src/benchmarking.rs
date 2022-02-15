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

fn init<T: Config>() {
	let default_balances: BalanceOf<T> = 1000000u64.saturated_into();
	let owner: T::AccountId = T::OwnerAddress::get();
	T::Currency::make_free_balance_be(&owner, default_balances);
	let import_owner: T::AccountId = T::ImportAddress::get();
	T::Currency::make_free_balance_be(&import_owner, default_balances);
	let user: T::AccountId = whitelisted_caller();
	T::Currency::make_free_balance_be(&user, default_balances);
}

benchmarks! {
	sign_up {
		init::<T>();
		let user: T::AccountId = whitelisted_caller();
		let account = "s234567";
        let account_vec = account.as_bytes().to_vec();
        let referrer_account = "gametop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
	}: _(RawOrigin::Signed(user), account_vec, referrer_account_vec)
	verify {
		
	}

	stake {
		init::<T>();
		let user: T::AccountId = whitelisted_caller();
		let amount: u64 = 1;
        let account = "s234567";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "gametop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        assert_ok!(Pallet::<T>::sign_up(RawOrigin::Signed(user.clone()).into(), account_vec.clone(), referrer_account_vec.clone()));
	}: _(RawOrigin::Signed(user), amount.saturated_into())
	verify {
		
	}

	unlock {
		init::<T>();
		let owner: T::AccountId = T::OwnerAddress::get();
		let user: T::AccountId = whitelisted_caller();
		let amount: u64 = 1;
        let account = "s234567";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "gametop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        assert_ok!(Pallet::<T>::sign_up(RawOrigin::Signed(user.clone()).into(), account_vec.clone(), referrer_account_vec.clone()));
        assert_ok!(Pallet::<T>::stake(RawOrigin::Signed(user.clone()).into(), amount.saturated_into()));
	}: _(RawOrigin::Signed(owner), user.clone(), amount.saturated_into())
	verify {
		
	}

	withdraw {
		init::<T>();
		let owner: T::AccountId = T::OwnerAddress::get();
		let user: T::AccountId = whitelisted_caller();
        let amount: u64 = 1;
        let account = "s234567";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "gametop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        assert_ok!(Pallet::<T>::sign_up(RawOrigin::Signed(user.clone()).into(), account_vec.clone(), referrer_account_vec.clone()));
	}: _(RawOrigin::Signed(owner), user.clone(), amount.saturated_into())
	verify {
		
	}

	import_stake {
		init::<T>();
		let owner: T::AccountId = T::OwnerAddress::get();
		let user: T::AccountId = whitelisted_caller();
        let amount: u64 = 1;
        let account = "s234567";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "gametop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        assert_ok!(Pallet::<T>::sign_up(RawOrigin::Signed(user.clone()).into(), account_vec.clone(), referrer_account_vec.clone()));
	}: _(RawOrigin::Signed(owner), user.clone(), amount.saturated_into())
	verify {
		
	}

	delete_user {
		init::<T>();
		let owner: T::AccountId = T::OwnerAddress::get();
		let user: T::AccountId = whitelisted_caller();
        let account = "s234567";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "gametop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        assert_ok!(Pallet::<T>::sign_up(RawOrigin::Signed(user.clone()).into(), account_vec.clone(), referrer_account_vec.clone()));
	}: _(RawOrigin::Signed(owner), user.clone(), account_vec.clone())
	verify {
		
	}
}

impl_benchmark_test_suite!(
	Pallet,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);
