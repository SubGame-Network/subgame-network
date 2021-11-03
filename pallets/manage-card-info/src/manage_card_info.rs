//! # Lease Interface
//!
//!  A list of leasable modules is defined in pallet-lease, and users can obtain module usage rights by staking nft tokens.
//!


use sp_std::{vec::Vec};
use sp_runtime::{RuntimeDebug};
use frame_support::{
    dispatch::{DispatchResult},
};
use sp_std::{prelude::*};

use codec::{Encode, Decode};

/// ability increases with level(the increased ability is a random number in the interval)
#[derive(Clone, Encode, Decode, Copy, Eq, PartialEq, RuntimeDebug, Default)]
pub struct AbilityOfLevel {
	/// type id
	// type_id: u128,
	/// level 
	pub level: u8,
	/// The card ability falls between min and max
	pub ability_value_1_min: u32,
	pub ability_value_1_max: u32,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct CardType<
	AbilityOfLevel,
	AccountId,
> {
	/// Can change `card`, `card_type`, `freezer` and `admin` accounts. 
	pub admin: AccountId,
	/// type id
	pub id: u128,
	/// type name
	pub name: Vec<u8>,
	/// type desc
	pub desc: Vec<u8>,
	/// special abilities/special skills
	pub fixed_ability_value_1: u32,
	pub fixed_ability_value_2: u32,
	/// special attributes: ex. earth/fire/water/air
	pub special_attribute_1: Vec<u8>,
	/// level limit
	pub level_max_limit: u32,
	/// ability increases with level(the increased ability is a random number in the interval)
	pub ability_of_level: Vec<AbilityOfLevel>,
	/// type draw
	pub is_can_draw: bool,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct CardInfo<TypeId> {
	/// The card info id
	pub id: u128,
	/// card info name
	pub name: Vec<u8>,
	/// card info desc
	pub desc: Vec<u8>,
	/// The card info type.
	pub type_id: TypeId,
}
pub trait ManageCardInfo<AccountId> {
    // type AbilityOfLevel = AbilityOfLevel;
    // type CardType = CardType;
    // type CardInfo = CardInfo;
    // type Card;
    fn _create_type(
		admin: AccountId,
		name: Vec<u8>,
		desc: Vec<u8>,
		fixed_ability_value_1: u32,
		fixed_ability_value_2: u32,
		special_attribute_1:Vec<u8>,
		level_max_limit: u32,
		ability_of_level: Vec<AbilityOfLevel>,
		is_can_draw: bool,
	) -> DispatchResult;

	fn _update_type(
		admin: AccountId,
		id: u128,
		name: Vec<u8>,
		desc: Vec<u8>,
		fixed_ability_value_1: u32,
		fixed_ability_value_2: u32,
		special_attribute_1:Vec<u8>,
		level_max_limit: u32,
		ability_of_level: Vec<AbilityOfLevel>,
		is_can_draw: bool,
	) -> DispatchResult;

	fn _create_card_info(
		admin: AccountId,
		name: Vec<u8>,
		desc: Vec<u8>,
		type_id: u128,
	) -> DispatchResult;

	fn _update_card_info(
		admin: AccountId,
		info_id: u128,
		name: Vec<u8>,
		desc: Vec<u8>,
	) -> DispatchResult;

	/// card type
	fn _get_card_types(
		id: u128,
	) -> Option<CardType<AbilityOfLevel, AccountId>>;

	/// card info
	fn _get_card_infos(
		id: u128,
	) -> Option<CardInfo<u128>>;
    
}
