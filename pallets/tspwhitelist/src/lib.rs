//! SubGame TSP Whitelist Dapp

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
	traits::{Currency, ReservableCurrency, ExistenceRequirement, Get, Vec},
	weights::{Weight, DispatchClass, Pays},
};
use frame_support::sp_std::convert::{TryInto};
use sp_runtime::{
	MultiSignature, AccountId32,
	traits::{
		SaturatedConversion, IdentifyAccount, Verify,
	}
};
use frame_system::ensure_signed;
use pallet_subgame_assets::{self as SubGameAssets};
use pallet_timestamp::{self as PalletTimestamp};

#[allow(unused_imports)]
use num_traits::float::FloatCore;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod default_weight;

#[macro_use]
extern crate alloc;

pub trait WeightInfo {
	fn whitelist() -> Weight;
	fn on_finalize() -> Weight;
}

/// SGB token decimals
pub const SGB_DECIMALS: u64 = 10_000_000_000;
/// TSP token decimals
pub const TSP_DECIMALS: u64 = 1_000_000;

pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

pub trait Config: frame_system::Config + SubGameAssets::Config + PalletTimestamp::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type WeightInfo: WeightInfo;
	type Currency: ReservableCurrency<Self::AccountId>;
	type OwnerAddress: Get<Self::AccountId>;
}

decl_storage! {
	trait Store for Module<T: Config> as TSPWhitelist {
		pub WhitelistStore get(fn whitelist_store): map hasher(blake2_128_concat) T::AccountId => T::SGAssetBalance;
		pub WhitelistAccount get(fn whitelist_account): Vec<T::AccountId>;
		pub WhitelistReceiveSGB get(fn whitelist_receive_sgb): T::SGAssetBalance;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
		SGAssetBalance = <T as SubGameAssets::Config>::SGAssetBalance,
	{
		Whitelist(AccountId, SGAssetBalance),
		AddWhitelist(AccountId),
		DelWhitelist(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		WhitelistNotStart,
		WhitelistEnd,
		BuyTooLittle,
		NotEnoughBalance,
		BuyTooMuch,
		AlradyWhitelist,
		NotWhitelist,
		PermissionDenied,
		WhitelistExists,
		WhitelistNotFound,
		TSPNotEnough,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = <T as Config>::WeightInfo::whitelist()]
		pub fn whitelist(origin, amount: T::SGAssetBalance) -> dispatch::DispatchResult
		{
			let sender = ensure_signed(origin)?;
			let owner = T::OwnerAddress::get();

			// Check whitelist time end
			let _now = PalletTimestamp::Pallet::<T>::get();
			let time_now = (TryInto::<u64>::try_into(_now).ok().unwrap()) as u64 / 1000u64;
			let event_start_time = 1639569600u64;
			let event_end_time = 1639828800u64;
			ensure!(time_now >= event_start_time, Error::<T>::WhitelistNotStart);
			ensure!(time_now <= event_end_time, Error::<T>::WhitelistEnd);

			// Check whitelist sgb limit end
			ensure!(WhitelistReceiveSGB::<T>::get().saturated_into::<u64>() < 2000000u64 * SGB_DECIMALS, Error::<T>::WhitelistEnd);

			// Check enough balance
			ensure!(amount.saturated_into::<u64>() >= 1u64 * SGB_DECIMALS, Error::<T>::BuyTooLittle);
			ensure!(amount.saturated_into::<u64>() % SGB_DECIMALS == 0u64, Error::<T>::BuyTooLittle);
			ensure!(<T as Config>::Currency::free_balance(&sender).saturated_into::<u64>() >= amount.saturated_into::<u64>(), Error::<T>::NotEnoughBalance);

			// buy max limit
			let _max_sgb: u64 = 250 * SGB_DECIMALS;
			
			// Check whitelist record
			let record_buy_amount = WhitelistStore::<T>::get(sender.clone());
			ensure!(record_buy_amount.saturated_into::<u64>() < _max_sgb, Error::<T>::AlradyWhitelist);
			
			// Check buy too much
			let total_buy = record_buy_amount + amount;
			ensure!(total_buy.saturated_into::<u64>() <= _max_sgb, Error::<T>::BuyTooMuch);

			// Check is whitelist
			let mut is_whitelist = false;
			let whitelist = WhitelistAccount::<T>::get();
			for addr in whitelist {
				let _account = T::AccountId::decode(&mut &addr.encode()[..]).unwrap_or_default();
				if _account == sender.clone() {
					is_whitelist = true;
					break;
				}
			}
			ensure!(is_whitelist == true, Error::<T>::NotWhitelist);

			let tsp_id: T::AssetId = 1001u32.into();
			let tsp_rate: u64 = 50;
			let whitelist_tsp: u64 = amount.saturated_into::<u64>() / SGB_DECIMALS * tsp_rate * TSP_DECIMALS;
			let tsp_balance: u64 = SubGameAssets::Module::<T>::balance(tsp_id, owner.clone()).saturated_into::<u64>();
			ensure!(tsp_balance >= whitelist_tsp, Error::<T>::TSPNotEnough);

			// transfer SGB to SubGame
			let _amount: u64 = amount.saturated_into::<u64>();
			<T as Config>::Currency::transfer(&sender, &owner, _amount.saturated_into(), ExistenceRequirement::KeepAlive)?;
			WhitelistStore::<T>::insert(sender.clone(), total_buy);
			WhitelistReceiveSGB::<T>::put(WhitelistReceiveSGB::<T>::get() + amount);

			// transfer TSP to user
			SubGameAssets::Module::<T>::_transfer(owner.clone(), tsp_id, sender.clone(), whitelist_tsp.saturated_into())?;
			
			Self::deposit_event(RawEvent::Whitelist(sender.clone(), amount));
			Ok(())
		}

		fn on_initialize(_now: T::BlockNumber) -> Weight {
            let mut whitelist = WhitelistAccount::<T>::get();
			if whitelist.len() == 0 {
				for addr in Self::default_whitelist() {
					let _account = T::AccountId::decode(&mut &addr.encode()[..]).unwrap_or_default();
					whitelist.push(_account);
				}
				WhitelistAccount::<T>::put(whitelist);
			}

			<T as Config>::WeightInfo::on_finalize()
        }

		#[weight = (10_000, DispatchClass::Normal, Pays::No)]
		pub fn add_whitelist(origin, account: T::AccountId) -> dispatch::DispatchResult
		{
			let sender = ensure_signed(origin)?;
			let owner = T::OwnerAddress::get();
			ensure!(owner == sender, Error::<T>::PermissionDenied);

			let mut check_status = true;
			let mut whitelist = WhitelistAccount::<T>::get();
			for addr in whitelist.clone() {
				let _old = T::AccountId::decode(&mut &addr.encode()[..]).unwrap_or_default();
				if _old == account {
					check_status = false;
					break;
				}
			}
			ensure!(check_status == true, Error::<T>::WhitelistExists);

			whitelist.push(account.clone());
			WhitelistAccount::<T>::put(whitelist);
			
			Self::deposit_event(RawEvent::AddWhitelist(account));
			Ok(())
		}

		#[weight = (10_000, DispatchClass::Normal, Pays::No)]
		pub fn del_whitelist(origin, account: T::AccountId) -> dispatch::DispatchResult
		{
			let sender = ensure_signed(origin)?;
			let owner = T::OwnerAddress::get();
			ensure!(owner == sender, Error::<T>::PermissionDenied);

			let mut check_status = false;
			let mut whitelist = WhitelistAccount::<T>::get();
			let mut loop_k: usize = 0;
			for addr in whitelist.clone() {
				let _old = T::AccountId::decode(&mut &addr.encode()[..]).unwrap_or_default();
				if _old == account {
					check_status = true;
					break;
				}
				loop_k = loop_k + 1;
			}
			ensure!(check_status == true, Error::<T>::WhitelistNotFound);

			whitelist.remove(loop_k);
			WhitelistAccount::<T>::put(whitelist);
			
			Self::deposit_event(RawEvent::DelWhitelist(account));
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	fn default_whitelist() -> Vec<AccountId32> {
		vec![
			// X
			AccountId::from(hex_literal::hex!("8822ecbae03679adf7fda0d0ac5ff34ab7b6fd3e8e8f16af5465e20372a63419")),

			// J
			AccountId::from(hex_literal::hex!("e03569d0f3af57ebc77c369f07d6d01b8774e271a69561e2838710b326f34300")),

			// R
			AccountId::from(hex_literal::hex!("3ee93b7abc479af0e1f90a23090f4342a2b4f5f9136eab2f24c07513ca703e6a")),
			AccountId::from(hex_literal::hex!("b8e9fe3ed2d4bb08457c264e4bf8371f849c9cf911698d05bc185e73ae259a73")),
			AccountId::from(hex_literal::hex!("6412b60805c2f0b2f56a6769a584dfc5b791d609f7db750bc0162c59ba5af633")),
			AccountId::from(hex_literal::hex!("5860f08b62a3c6026f43350e2cfc433c51779349f33153fa6589bea594304d63")),
			AccountId::from(hex_literal::hex!("20d096f7e7f862bfea810588dc4b0ba5e5d57eb0b52f16bd5dd7d2100eb52710")),

			// D
			AccountId::from(hex_literal::hex!("586f751d0047ca1ac6e9a70f4993cab5c86db004a881d1c634986665f620d95b")),
			AccountId::from(hex_literal::hex!("6e91e2431c5905a2d2a0b46ab8eedf4057f0ec6e9fb6cae3f78ecd8435ed3f6c")),
			AccountId::from(hex_literal::hex!("2e3a82d28d4cafc7ea6b4d47fe0afbde6719dccc7e892ede840a93abb2029968")),
			AccountId::from(hex_literal::hex!("c2749042c2f834d0986d5df9c75f9bf9ce6657dff78a6faafe1c19a3caf6b363")),
			AccountId::from(hex_literal::hex!("b81326928f23bbde9abbee107bdfcd60dbd7f5a7b7b15d97f2ad52253f301634")),
		]
	}
}