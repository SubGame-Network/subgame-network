//! # Lease Interface
//!
//!  A list of leasable modules is defined in pallet-lease, and users can obtain module usage rights by staking nft tokens.
//!


use sp_runtime::{RuntimeDebug};
use frame_support::{
    dispatch::{DispatchResult},
};
use sp_std::{prelude::*};

use codec::{Encode, Decode};

/// ability increases with level(the increased ability is a random number in the interval)
#[derive(Clone, Encode, Decode, Copy, Eq, PartialEq, RuntimeDebug, Default)]
pub struct Platform<Account> {
	pub id: u128,
	pub admin: Account,
	pub percentage_of_fee: u8,
	pub fee_account: Account,
}

#[derive(Clone, Encode, Decode, Copy, Eq, PartialEq, RuntimeDebug, Default)]
pub struct Auction<Account, NftId, BalanceOf> {
	pub id: u128,
	pub platform_id: u128,
	pub nft_id: NftId,
	pub seller: Account,
	pub buyer: Option<Account>,
	pub amount: BalanceOf,
	pub percentage_of_fee: u8,
	pub platform_fee: BalanceOf,
}

pub trait NftExchange<AccountId, NftId, BalanceOf> {
	fn _create_platform(
		admin: AccountId,
		percentage_of_fee: u8,
		fee_account: AccountId,
	) -> DispatchResult;

	fn _update_platform(
		admin: AccountId,
		id: u128,
		percentage_of_fee: u8,
		fee_account: AccountId,
	) -> DispatchResult;

	fn _create_auction(
		platform_id: u128,
		seller: AccountId,
		nft_id: NftId,
		amount: BalanceOf,
	) -> DispatchResult;

	fn _auction_buy(
		auction_id: u128,
		buyer: AccountId,
	) -> DispatchResult;

	fn _auction_done(
		auction_id: u128,
		owner: AccountId,
	) -> DispatchResult;
}
