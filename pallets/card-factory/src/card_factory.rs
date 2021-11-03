//! # Lease Interface
//!
//!  A list of leasable modules is defined in pallet-lease, and users can obtain module usage rights by staking nft tokens.
//!


use sp_std::{vec::Vec};


use frame_support::{
    dispatch::{DispatchResult},
};
pub trait CardFactory<AccountId, NftId> {
    type Card;
   
	/// card
	fn _create_card(
		admin: AccountId,
		card_info_id: u128,
		level: u8,
	) -> DispatchResult;

	/// admin can edit card
	fn _edit_card(
		admin: AccountId,
		card_id: u128,
		level: u8,
		ability_value_1: u32,
	) -> DispatchResult;

	/// card
	fn _destroy_card(
		admin: AccountId,
		card_id: u128,
	) -> DispatchResult;

	/// card
	fn _get_user_cards(
		owner: AccountId,
	) -> Vec<Self::Card>;
    
}
