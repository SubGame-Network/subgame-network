#![cfg_attr(not(feature = "std"), no_std)]

pub mod default_weight;
use default_weight::WeightInfo;

use sp_std::{prelude::*};
use frame_support::{decl_module, decl_event, decl_storage, decl_error, ensure,
	traits::{Get},
	dispatch::{DispatchResult},
};
use frame_system::ensure_signed;

pub mod manage_card_info;
pub use crate::manage_card_info::*;


use pallet_lease::Lease;

use pallet_nft::UniqueAssets;

pub type NftId<T> = <<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetId;

pub type PalletId<T> = 
    <<T as Config>::Lease as Lease<<T as frame_system::Config>::AccountId, NftId<T>>>::PalletId;

/// The module configuration trait.
pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	
    type UniqueAssets: UniqueAssets<Self::AccountId>;
    type Lease: Lease<Self::AccountId, NftId<Self>>;
    type PalletId: Get<PalletId<Self>>;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}



decl_storage! {
	trait Store for Module<T: Config> as ManageCardInfo {
        pub NextCardInfoId get(fn next_card_info_id): u128 = 1;
		pub NextCardTypeId get(fn next_card_type_id): u128 = 1;

		pub CardInfos get(fn card_info_by_id): map hasher(blake2_128_concat) u128 => Option<CardInfo<u128>>;
		pub CardTypes get(fn card_type_by_id): map hasher(blake2_128_concat) u128 => Option<CardType<AbilityOfLevel, T::AccountId>>;
	}
}

decl_event! {
	pub enum Event<T> where
		AccountId = <T as frame_system::Config>::AccountId,
		AbilityOfLevel = AbilityOfLevel,
	{
		NewCardType(AccountId,u128,Vec<u8>,Vec<u8>,u32,u32,u32,Vec<AbilityOfLevel>,bool),
		UpdateCardType(u128,Vec<u8>,Vec<u8>,u32,u32,u32,Vec<AbilityOfLevel>,bool),
		NewCardInfo(AccountId,u128,Vec<u8>,Vec<u8>,u128),
		UpdateCardInfo(u128,Vec<u8>,Vec<u8>),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
	
		AbilityOfLevelNotMatchLimit,
		PermissionDenied,
		NotAdmin,
		UnknownType,
		NotFoundData,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		
		#[weight = T::WeightInfo::create_type()]
		fn create_type(origin,
			name: Vec<u8>,
			desc: Vec<u8>,
			fixed_ability_value_1: u32,
			fixed_ability_value_2: u32,
			special_attribute_1:Vec<u8>,
			level_max_limit: u32,
			ability_of_level: Vec<AbilityOfLevel>,
			is_can_draw: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Self::_create_type(sender, 
				name, 
				desc, 
				fixed_ability_value_1, 
				fixed_ability_value_2, 
				special_attribute_1, 
				level_max_limit, 
				ability_of_level, 
				is_can_draw
			)
		}

		#[weight = T::WeightInfo::update_type()]
		fn update_type(origin,
			id: u128,
			name: Vec<u8>,
			desc: Vec<u8>,
			fixed_ability_value_1: u32,
			fixed_ability_value_2: u32,
			special_attribute_1:Vec<u8>,
			level_max_limit: u32,
			ability_of_level: Vec<AbilityOfLevel>,
			is_can_draw: bool,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Self::_update_type(sender, 
				id,
				name,
				desc,
				fixed_ability_value_1,
				fixed_ability_value_2,
				special_attribute_1,
				level_max_limit,
				ability_of_level,
				is_can_draw,
			)
		}

		#[weight = T::WeightInfo::change_admin()]
		fn change_admin(origin,
			type_id: u128,
			new_admin: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			CardTypes::<T>::try_mutate_exists(type_id, |card_type| {
				let _card_type = card_type.as_mut().ok_or( Error::<T>::NotFoundData)?;
				ensure!(_card_type.admin == sender, Error::<T>::NotAdmin);
				_card_type.admin = new_admin;
				Ok(())
			})
		}

		#[weight = T::WeightInfo::create_card_info()]
		fn create_card_info(origin,
			name: Vec<u8>,
			desc: Vec<u8>,
			type_id: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Self::_create_card_info(sender, 
				name,
				desc,
				type_id,
			)
		}

		#[weight = T::WeightInfo::update_card_info()]
		fn update_card_info(origin,
			info_id: u128,
			name: Vec<u8>,
			desc: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Self::_update_card_info(sender, 
				info_id,
				name,
				desc,
			)
		}
	}
}

// The main implementation block for the module.
impl<T: Config> ManageCardInfo<T::AccountId> for Module<T> {
	// Public immutables
	fn _create_type(
		admin: T::AccountId,
		name: Vec<u8>,
		desc: Vec<u8>,
		fixed_ability_value_1: u32,
		fixed_ability_value_2: u32,
		special_attribute_1:Vec<u8>,
		level_max_limit: u32,
		ability_of_level: Vec<AbilityOfLevel>,
		is_can_draw: bool,
	) -> DispatchResult {
		
		let type_id = Self::next_card_type_id();

		
		ensure!(ability_of_level.len() == level_max_limit as usize, Error::<T>::AbilityOfLevelNotMatchLimit);

		CardTypes::<T>::insert(type_id, CardType {
			admin: admin.clone(),
			id: type_id,
			name: name.clone(),
			desc: desc.clone(),
			special_attribute_1: special_attribute_1,
			fixed_ability_value_1: fixed_ability_value_1,
			fixed_ability_value_2: fixed_ability_value_2,
			level_max_limit: level_max_limit,
			ability_of_level: ability_of_level.clone(),
			is_can_draw: is_can_draw,
		});

        NextCardTypeId::mutate(|card_type_id| *card_type_id += 1);

		Self::deposit_event(RawEvent::NewCardType(
			admin,
			type_id,
			name,
			desc,
			fixed_ability_value_1,
			fixed_ability_value_2,
			level_max_limit,
			ability_of_level,
			is_can_draw,
		));
		Ok(())
	}

	fn _update_type(
		admin: T::AccountId,
		id: u128,
		name: Vec<u8>,
		desc: Vec<u8>,
		fixed_ability_value_1: u32,
		fixed_ability_value_2: u32,
		special_attribute_1: Vec<u8>,
		level_max_limit: u32,
		ability_of_level: Vec<AbilityOfLevel>,
		is_can_draw: bool,
	) -> DispatchResult {
		
		ensure!(ability_of_level.len() == level_max_limit as usize, Error::<T>::AbilityOfLevelNotMatchLimit);
		CardTypes::<T>::try_mutate_exists(id, |card_type| {
			let _card_type = card_type.take().ok_or( Error::<T>::NotFoundData)?;
			ensure!(_card_type.admin == admin, Error::<T>::NotAdmin);

			*card_type = Some(CardType {
				admin: admin,
				id: id,
				name: name.clone(),
				desc: desc.clone(),
				special_attribute_1: special_attribute_1,
				fixed_ability_value_1: fixed_ability_value_1,
				fixed_ability_value_2: fixed_ability_value_2,
				level_max_limit: level_max_limit,
				ability_of_level: ability_of_level.clone(),
				is_can_draw: is_can_draw,
			});

			Self::deposit_event(RawEvent::UpdateCardType(
				id,
				name,
				desc,
				fixed_ability_value_1,
				fixed_ability_value_2,
				level_max_limit,
				ability_of_level,
				is_can_draw,
			));
			Ok(())
		})
	}

	fn _create_card_info(
		admin: T::AccountId,
		name: Vec<u8>,
		desc: Vec<u8>,
		type_id: u128,
	) -> DispatchResult {
		// check type exist & sender is admin
		let _type = CardTypes::<T>::get(type_id).ok_or(Error::<T>::UnknownType)?;
		ensure!(admin == _type.admin, Error::<T>::NotAdmin);

		let info_id = Self::next_card_info_id();

		CardInfos::insert(info_id, CardInfo {
			id: info_id,
			name: name.clone(),
			desc: desc.clone(),
			type_id: type_id,
		});

        NextCardInfoId::mutate(|card_info_id| *card_info_id += 1);

		Self::deposit_event(RawEvent::NewCardInfo(
			admin,
			info_id,
			name.clone(),
			desc.clone(),
			type_id,
		));
		Ok(())
	}

	fn _update_card_info(
		admin: T::AccountId,
		info_id: u128,
		name: Vec<u8>,
		desc: Vec<u8>,
	) -> DispatchResult {

		CardInfos::try_mutate_exists(info_id, |card_info| {
			// check info exist
			let _card_info = card_info.take().ok_or(Error::<T>::NotFoundData)?;
			
			// check type exist & sender is admin
			let _type = CardTypes::<T>::get(_card_info.type_id).ok_or(Error::<T>::UnknownType)?;
			ensure!(admin == _type.admin, Error::<T>::NotAdmin);
			
			*card_info = Some(CardInfo {
				id: info_id,
				name: name.clone(),
				desc: desc.clone(),
				type_id: _type.id,
			});

			Self::deposit_event(RawEvent::UpdateCardInfo(
				info_id,
				name,
				desc,
			));
			Ok(())
		})
	}

	/// card type
	fn _get_card_types(
		id: u128,
	) -> Option<CardType<AbilityOfLevel, T::AccountId>>  {
		CardTypes::<T>::get(id)
	}

	/// card info
	fn _get_card_infos(
		id: u128,
	) -> Option<CardInfo<u128>>  {
		CardInfos::get(id)
	}
}
