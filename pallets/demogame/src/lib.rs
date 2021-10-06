//! Management of Game Template instances, can create game and bet, query the specified template,  current betting games and historical games
#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{Get},
};
use frame_system::ensure_signed;
use pallet_lease::Lease;

use pallet_nft::UniqueAssets;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type NftId<T> = 
<<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetId;
pub type PalletId<T> = 
    <<T as Config>::Lease as Lease<<T as frame_system::Config>::AccountId, NftId<T>>>::PalletId;
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type UniqueAssets: UniqueAssets<Self::AccountId>;
    type Lease: Lease<Self::AccountId, NftId<Self>>;
    type PalletId: Get<PalletId<Self>>;
}

decl_storage! {
    trait Store for Module<T: Config> as GameCenterModule {
        /// Call Success count
        pub CallSuccess get(fn call_success): map hasher(blake2_128_concat) T::AccountId=> u64 = 0;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
    {
        CallSuccess(AccountId),
    }
);
decl_error! {
    pub enum Error for Module<T: Config> {
        PermissionDenied
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// play template game
        #[weight = 10_000]
        pub fn demo(origin) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);

            CallSuccess::<T>::mutate(sender.clone(), |total| *total += 1);
            
            Self::deposit_event(RawEvent::CallSuccess(sender.clone()));
            Ok(())
        }
    }
}