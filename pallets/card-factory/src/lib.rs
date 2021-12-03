#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod default_weight;
use default_weight::WeightInfo;

use frame_system::ensure_signed;
use sp_std::{prelude::*};
use sp_runtime::RuntimeDebug;
// use frame_support::traits::Randomness;

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_event, decl_storage, decl_error, ensure,
	traits::{Get},
	dispatch::{DispatchResult},
};

pub mod card_factory;
pub use crate::card_factory::CardFactory;

use pallet_lease::Lease;

use pallet_manage_card_info::ManageCardInfo;

use sp_runtime::traits::{Hash, BlakeTwo256};
use sp_runtime::RandomNumberGenerator;

use pallet_nft::UniqueAssets;

pub type NftId<T> = <<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetId;

pub type PalletId<T> = 
    <<T as Config>::Lease as Lease<<T as frame_system::Config>::AccountId, NftId<T>>>::PalletId;

/// The module configuration trait.
pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	// type MyRandomness: Randomness<Self::Hash>;

    type UniqueAssets: UniqueAssets<Self::AccountId>;
    type Lease: Lease<Self::AccountId, NftId<Self>>;
    type ManageCardInfo: ManageCardInfo<Self::AccountId>;
    type PalletId: Get<PalletId<Self>>;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}

// 實際的卡片
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct Card<NftId> {
	/// The card id
	id: u128,
	card_info_id: u128,
	level: u8,
	ability_value_1: u32,
	/// The card nft id
	nft_id: NftId,
}


decl_storage! {
	trait Store for Module<T: Config> as CardFactory {
		pub NextCardId get(fn next_card_id): u128 = 1;
		// Card entity
		pub Cards get(fn card_by_id): map hasher(blake2_128_concat) u128 => Card<NftId<T>>;
		// get Card entity by nftId
		pub CardsByNftId get(fn card_by_nftid): map hasher(blake2_128_concat) NftId<T> => u128;
	}
}

decl_event! {
	pub enum Event<T> where
		AccountId = <T as frame_system::Config>::AccountId,
		NftId = NftId<T>,
	{
		NewCard(AccountId, u128, u128, u8, u32, NftId),
		DestroyCard(AccountId, u128),
		UpdateCard(AccountId, u128, u128, u8, u32, NftId),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		AbilityOfLevelNotMatchLimit,
		NotAdmin,
		UnknownType,
		NotFoundData,
		NotCardOwner,
		PermissionDenied,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;
		
		#[weight = T::WeightInfo::create_card()]
		fn create_card(origin,
			card_info_id: u128,
			level: u8,
		) -> DispatchResult {
			let admin = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), admin.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			
			Self::_create_card(
				admin,
				card_info_id,
				level,
			)
		}
		
		#[weight = T::WeightInfo::edit_card()]
		fn edit_card(origin,
			card_id: u128,
			level: u8,
			ability_value_1: u32,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Self::_edit_card(
				sender,
				card_id,
				level,
				ability_value_1,
			)
		}
		
		#[weight = T::WeightInfo::destroy_card()]
		fn destroy_card(origin,
			card_id: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Self::_destroy_card(
				sender,
				card_id,
			)
		}
		
	}
}

// The main implementation block for the module.
impl<T: Config> CardFactory<T::AccountId, NftId<T>> for Module<T> {
    type Card = Card<NftId<T>>;
	// Public immutables

	/// card
	fn _create_card(
		admin: T::AccountId,
		card_info_id: u128,
		level: u8,
	) -> DispatchResult {
		let _card_info = T::ManageCardInfo::_get_card_infos(card_info_id).ok_or(Error::<T>::UnknownType)?;
		let _card_type = T::ManageCardInfo::_get_card_types(_card_info.type_id).ok_or(Error::<T>::UnknownType)?;
		ensure!(admin == _card_type.admin, Error::<T>::NotAdmin);


		let id = Self::next_card_id();

		ensure!(_card_type.ability_of_level.len() >= level as usize, Error::<T>::AbilityOfLevelNotMatchLimit);

		let nft_id = T::UniqueAssets::mint(&admin, Vec::new())?;
		let ability_of_level = _card_type.ability_of_level[(level-1) as usize];

		let ability_max = ability_of_level.ability_value_1_max;
		let ability_min = ability_of_level.ability_value_1_min;
			
		
		// let a = T::MyRandomness::random(&(T::PalletId::get(), 100u32).encode());
		// let random_seed = sp_io::offchain::random_seed();
		

		let ability_value_1: &u32;
		let mut range = Vec::new();
		// let mut rng = rand::thread_rng();
		// let die = Uniform::from(ability_min..ability_max);
		// let ability_value_1 = die.sample(&mut rng);
		if ability_max <= ability_min {
			ability_value_1 = &ability_max;
		}else{
			let random_seed = BlakeTwo256::hash(&(id).encode());
			let mut rng = RandomNumberGenerator::<BlakeTwo256>::new(random_seed);
			for n in ability_min..ability_max{
				range.push(n);
			}
			let ability_value_2 = rng.pick_item(&range).unwrap();
			ability_value_1  =  ability_value_2;
		}

		
		
		Cards::<T>::insert(id, Card {
			id: id,
			card_info_id: card_info_id,
			level: level,
			ability_value_1: *ability_value_1,
			nft_id: nft_id.clone(),
		});

		CardsByNftId::<T>::insert(nft_id.clone(), id);

        NextCardId::mutate(|card_info_id| *card_info_id += 1);

		Self::deposit_event(RawEvent::NewCard(
			admin,
			id,
			card_info_id,
			level,
			*ability_value_1,
			nft_id,
		));
		Ok(())
	}

	/// admin can edit card
	fn _edit_card(
		admin: T::AccountId,
		card_id: u128,
		level: u8,
		ability_value_1: u32,
	) -> DispatchResult {
		let _card = Cards::<T>::get(card_id);
		ensure!(_card.id != 0, Error::<T>::NotFoundData);
		let _card_info = T::ManageCardInfo::_get_card_infos(_card.card_info_id).ok_or(Error::<T>::UnknownType)?;
		let _card_type = T::ManageCardInfo::_get_card_types(_card_info.type_id).ok_or(Error::<T>::UnknownType)?;

		ensure!(_card_type.admin == admin, Error::<T>::NotFoundData);

		Cards::<T>::try_mutate_exists(card_id, |card| {
			let _card = card.take().ok_or( Error::<T>::NotFoundData)?;

			*card = Some(Card {
				id: card_id,
				card_info_id: _card.card_info_id,
				level: level,
				ability_value_1: ability_value_1,
				nft_id: _card.nft_id.clone(),
			});

			Self::deposit_event(RawEvent::UpdateCard(
				admin,
				card_id,
				_card.card_info_id,
				level,
				ability_value_1,
				_card.nft_id,
			));
			Ok(())
		})
	}

	/// card
	fn _destroy_card(
		admin: T::AccountId,
		card_id: u128,
	) -> DispatchResult {
		let _card = Cards::<T>::get(card_id);
		ensure!(_card.id != 0, Error::<T>::NotFoundData);
		let _card_info = T::ManageCardInfo::_get_card_infos(_card.card_info_id).ok_or(Error::<T>::UnknownType)?;
		let _card_type = T::ManageCardInfo::_get_card_types(_card_info.type_id).ok_or(Error::<T>::UnknownType)?;

		ensure!(_card_type.admin == admin, Error::<T>::NotFoundData);

		let owner = T::UniqueAssets::owner_of(&_card.nft_id);
		ensure!(owner == admin, Error::<T>::NotCardOwner);

		// remove card
        Cards::<T>::remove(card_id);

		// remove 
		CardsByNftId::<T>::remove(_card.nft_id.clone());
		

		Self::deposit_event(RawEvent::DestroyCard(
			admin,
			card_id,
		));
		Ok(())
	}

	/// card
	fn _get_user_cards(
		owner: T::AccountId,
	) -> Vec<Card<NftId<T>>>  {
		let assets = T::UniqueAssets::assets_for_account(&owner);
		let mut _cards: Vec<Card<NftId<T>>> = Vec::new();
		for _asset in assets.iter() {

            let _nft_id = _asset.0.clone();
			let card_id = Self::card_by_nftid(_nft_id);
			let card = Self::card_by_id(card_id);
			if card.id == 0 {
				continue;
			}
			_cards.push(card);
		}

		_cards
	}
	
}
