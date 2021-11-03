#![cfg_attr(not(feature = "std"), no_std)]

pub mod default_weight;
use default_weight::WeightInfo;


use sp_std::{prelude::*};
use frame_support::{decl_module, decl_event, decl_storage, decl_error, ensure,
	traits::{Currency, Get},
	dispatch::{DispatchResult},
};
use frame_system::ensure_signed;


use pallet_lease::Lease;

use pallet_nft::UniqueAssets;

pub type NftId<T> = <<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetId;

pub type BalanceOf<T> =
    <<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type PalletId<T> = 
    <<T as Config>::Lease as Lease<<T as frame_system::Config>::AccountId, NftId<T>>>::PalletId;

/// The module configuration trait.
pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	
	type OwnerAddress: Get<Self::AccountId>;
    type Balances: Currency<Self::AccountId>;
    type UniqueAssets: UniqueAssets<Self::AccountId>;
    type Lease: Lease<Self::AccountId, NftId<Self>>;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}



decl_storage! {
	trait Store for Module<T: Config> as NftExchange {
       
	}
}

decl_event! {
	pub enum Event<T> where
		AccountId = <T as frame_system::Config>::AccountId,
		NftId = NftId<T>,
	{
		DrawCard(AccountId,NftId),
		SyntheticCards(AccountId,NftId,NftId,NftId),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		PermissionDenied,
		NotNftOwner,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		
		#[weight = T::WeightInfo::draw_card()]
		fn draw_card(origin,
			target: T::AccountId,
			card_nft_id: NftId<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
            let admin = T::OwnerAddress::get();
            ensure!(admin == sender, Error::<T>::PermissionDenied);
			
			ensure!(T::UniqueAssets::owner_of(&card_nft_id) == admin, Error::<T>::NotNftOwner);
			T::UniqueAssets::transfer(&target, &card_nft_id)?;
			Ok(())
		}

		#[weight = T::WeightInfo::synthetic_cards()]
		fn synthetic_cards(origin,
			target: T::AccountId,
			old_card_nft_id1: NftId<T>,
			old_card_nft_id2: NftId<T>,
			new_card_nft_id: NftId<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
            let admin = T::OwnerAddress::get();
            ensure!(admin == sender, Error::<T>::PermissionDenied);

			ensure!(T::UniqueAssets::owner_of(&old_card_nft_id1) == target, Error::<T>::NotNftOwner);
			ensure!(T::UniqueAssets::owner_of(&old_card_nft_id2) == target, Error::<T>::NotNftOwner);
			ensure!(T::UniqueAssets::owner_of(&new_card_nft_id) == admin, Error::<T>::NotNftOwner);

			T::UniqueAssets::burn(&old_card_nft_id1)?;
			T::UniqueAssets::burn(&old_card_nft_id2)?;

			T::UniqueAssets::transfer(&target, &new_card_nft_id)?;
			
			Ok(())
		}
		
	}
}