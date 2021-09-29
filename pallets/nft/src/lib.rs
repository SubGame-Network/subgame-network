//! # Unique Assets Implementation: Commodities
//!
//! This pallet exposes capabilities for managing unique assets, also known as
//! non-fungible tokens (NFTs).
//!
//! - [`pallet_commodities::Trait`](./trait.Trait.html)
//! - [`Calls`](./enum.Call.html)
//! - [`Errors`](./enum.Error.html)
//! - [`Events`](./enum.RawEvent.html)
//!
//! ## Overview
//!
//! Assets that share a common metadata structure may be created and distributed
//! by an asset admin. Asset owners may burn assets or transfer their
//! ownership. Configuration parameters are used to limit the total number of a
//! type of asset that may exist as well as the number that any one account may
//! own. Assets are uniquely identified by the hash of the info that defines
//! them, as calculated by the runtime system's hashing algorithm.
//!
//! This pallet implements the [`UniqueAssets`](./nft/trait.UniqueAssets.html)
//! trait in a way that is optimized for assets that are expected to be traded
//! frequently.
//!
//! ### Dispatchable Functions
//!
//! * [`mint`](./enum.Call.html#variant.mint) - Use the provided commodity info
//!   to create a new commodity for the specified user. May only be called by
//!   the commodity admin.
//!
//! * [`burn`](./enum.Call.html#variant.burn) - Destroy a commodity. May only be
//!   called by commodity owner.
//!
//! * [`transfer`](./enum.Call.html#variant.transfer) - Transfer ownership of
//!   a commodity to another account. May only be called by current commodity
//!   owner.

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{EnsureOrigin, Get},
};
use codec::{alloc::string::{ToString, String}};
// use alloc::string::{String, ToString};
use frame_system::ensure_signed;
use sp_runtime::traits::{Hash};
use sp_std::{cmp::Eq, vec::Vec};

pub mod nft;
pub use crate::nft::UniqueAssets;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
    /// The dispatch origin that is able to mint new instances of this type of commodity.
    type CommodityAdmin: EnsureOrigin<Self::Origin>;
    /// The maximum number of this type of commodity that may exist (minted - burned).
    type CommodityLimit: Get<u128>;
    /// The maximum number of this type of commodity that any single account may own.
    type UserCommodityLimit: Get<u64>;
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

/// The runtime system's hashing algorithm is used to uniquely identify commodities.
pub type CommodityId<T> = <T as frame_system::Config>::Hash;

/// Associates a commodity with its ID.
pub type Commodity<T> = (CommodityId<T>, Vec<u8>);

decl_storage! {
    trait Store for Module<T: Config> as Commodity {
        /// The total number of this type of commodity that exists (minted - burned).
        Total get(fn total): u128 = 0;
        /// It is an incremental number. When mint does not specify info, this will be the seed used to generate nft hash
        NextNfcId get(fn next_nfc_id): u128 = 0;
        /// The total number of this type of commodity that has been burned (may overflow).
        Burned get(fn burned): u128 = 0;
        /// The total number of this type of commodity owned by an account.
        TotalForAccount get(fn total_for_account): map hasher(blake2_128_concat) T::AccountId => u64 = 0;
        /// A mapping from an account to a list of all of the commodities of this type that are owned by it.
        CommoditiesForAccount get(fn commodities_for_account): map hasher(blake2_128_concat) T::AccountId => Vec<Commodity<T>>;
        /// A mapping from a commodity ID to the account that owns it.
        AccountForCommodity get(fn account_for_commodity): map hasher(identity) CommodityId<T> => T::AccountId;
    }

    add_extra_genesis {
        config(balances): Vec<(T::AccountId, Vec<Vec<u8>>)>;
        build(|config: &GenesisConfig<T>| {
            for (who, assets) in config.balances.iter() {
                for asset in assets {
                    match <Module::<T> as UniqueAssets::<T::AccountId>>::mint(who, asset.clone()) {
                        Ok(_) => {}
                        Err(err) => { std::panic::panic_any(err) },
                    }
                }
            }
        });
    }
}

decl_event!(
    pub enum Event<T>
    where
        CommodityId = <T as frame_system::Config>::Hash,
        AccountId = <T as frame_system::Config>::AccountId,
    {
        /// The commodity has been burned.
        Burned(CommodityId),
        /// The commodity has been minted and distributed to the account.
        Minted(CommodityId, AccountId),
        /// Ownership of the commodity has been transferred to the account.
        Transferred(CommodityId, AccountId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        // Thrown when there is an attempt to mint a duplicate commodity.
        CommodityExists,
        // Thrown when there is an attempt to burn or transfer a nonexistent commodity.
        NonexistentCommodity,
        // Thrown when someone who is not the owner of a commodity attempts to transfer or burn it.
        NotCommodityOwner,
        // Thrown when the commodity admin attempts to mint a commodity and the maximum number of this
        // type of commodity already exists.
        TooManyCommodities,
        // Thrown when an attempt is made to mint or transfer a commodity to an account that already
        // owns the maximum number of this type of commodity.
        TooManyCommoditiesForAccount,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// Create a new commodity from the provided commodity info and identify the specified
        /// account as its owner. The ID of the new commodity will be equal to the hash of the info
        /// that defines it, as calculated by the runtime system's hashing algorithm.
        ///
        /// The dispatch origin for this call must be the commodity admin.
        ///
        /// This function will throw an error if it is called with commodity info that describes
        /// an existing (duplicate) commodity, if the maximum number of this type of commodity already
        /// exists or if the specified owner already owns the maximum number of this type of
        /// commodity.
        ///
        /// - `owner_account`: Receiver of the commodity.
        /// - `commodity_info`: The information that defines the commodity.
        #[weight = 10_000]
        pub fn mint(origin, owner_account: T::AccountId, info: Vec<u8>) -> dispatch::DispatchResult {
            T::CommodityAdmin::ensure_origin(origin)?;
            let commodity_id = <Self as UniqueAssets<_>>::mint(&owner_account, info)?;
            Self::deposit_event(RawEvent::Minted(commodity_id, owner_account.clone()));
            Ok(())
        }

        /// Destroy the specified commodity.
        ///
        /// The dispatch origin for this call must be the commodity owner.
        ///
        /// - `commodity_id`: The hash (calculated by the runtime system's hashing algorithm)
        ///   of the info that defines the commodity to destroy.
        #[weight = 10_000]
        pub fn burn(origin, commodity_id: CommodityId<T>) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(who == Self::account_for_commodity(&commodity_id), Error::<T>::NotCommodityOwner);

            <Self as UniqueAssets<_>>::burn(&commodity_id)?;
            Self::deposit_event(RawEvent::Burned(commodity_id.clone()));
            Ok(())
        }

        /// Transfer a commodity to a new owner.
        ///
        /// The dispatch origin for this call must be the commodity owner.
        ///
        /// This function will throw an error if the new owner already owns the maximum
        /// number of this type of commodity.
        ///
        /// - `dest_account`: Receiver of the commodity.
        /// - `commodity_id`: The hash (calculated by the runtime system's hashing algorithm)
        ///   of the info that defines the commodity to destroy.
        #[weight = 10_000]
        pub fn transfer(origin, dest_account: T::AccountId, commodity_id: CommodityId<T>) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(who == Self::account_for_commodity(&commodity_id), Error::<T>::NotCommodityOwner);

            <Self as UniqueAssets<_>>::transfer(&dest_account, &commodity_id)?;
            Self::deposit_event(RawEvent::Transferred(commodity_id.clone(), dest_account.clone()));
            Ok(())
        }
    }
}

// impl From<str> for <T as Config>::CommodityInfo {
//     fn from(item: str) -> Self {
//         <T as Config>::CommodityInfo { value: item }
//     }
// }

impl<T: Config> UniqueAssets<T::AccountId> for Module<T> {
    type AssetId = CommodityId<T>;
    type AssetInfo = Vec<u8>;
    type AssetLimit = T::CommodityLimit;
    type UserAssetLimit = T::UserCommodityLimit;

    fn total() -> u128 {
        Self::total()
    }

    fn burned() -> u128 {
        Self::burned()
    }

    fn total_for_account(account: &T::AccountId) -> u64 {
        Self::total_for_account(account)
    }

    fn assets_for_account(account: &T::AccountId) -> Vec<Commodity<T>> {
        Self::commodities_for_account(account)
    }

    fn owner_of(commodity_id: &CommodityId<T>) -> T::AccountId {
        Self::account_for_commodity(commodity_id)
    }

    fn mint(
        owner_account: &T::AccountId,
        info: Vec<u8>,
    ) -> dispatch::result::Result<CommodityId<T>, dispatch::DispatchError> {
        // let commodity_info = TryInto::<u64>::try_into(now).ok().unwrap(); // convert to u64(time ms)
        let commodity_info: Vec<u8>;
        if info.is_empty() {
            let b = Self::next_nfc_id().to_string().into_bytes();
            commodity_info = b;
        } else {
            commodity_info = info;
        }
        

        let commodity_id = T::Hashing::hash_of(&commodity_info);

        ensure!(
            !AccountForCommodity::<T>::contains_key(&commodity_id),
            Error::<T>::CommodityExists
        );

        ensure!(
            Self::total_for_account(owner_account) < T::UserCommodityLimit::get(),
            Error::<T>::TooManyCommoditiesForAccount
        );

        ensure!(
            Self::total() < T::CommodityLimit::get(),
            Error::<T>::TooManyCommodities
        );

        let new_commodity = (commodity_id, commodity_info);

        Total::mutate(|total| *total += 1);
        NextNfcId::mutate(|nft_id| *nft_id += 1);
        TotalForAccount::<T>::mutate(owner_account, |total| *total += 1);
        CommoditiesForAccount::<T>::mutate(owner_account, |commodities| {
            match commodities.binary_search(&new_commodity) {
                Ok(_pos) => {} // should never happen
                Err(pos) => commodities.insert(pos, new_commodity),
            }
        });
        AccountForCommodity::<T>::insert(commodity_id, &owner_account);

        Ok(commodity_id)
    }

    fn burn(commodity_id: &CommodityId<T>) -> dispatch::DispatchResult {
        let owner = Self::owner_of(commodity_id);
        ensure!(
            owner != T::AccountId::default(),
            Error::<T>::NonexistentCommodity
        );

        Total::mutate(|total| *total -= 1);
        Burned::mutate(|total| *total += 1);
        TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);
        CommoditiesForAccount::<T>::mutate(owner, |commodities| {
            let pos = commodities
                .binary_search_by(|probe| probe.0.cmp(commodity_id))
                .expect("We already checked that we have the correct owner; qed");
            commodities.remove(pos);
        });
        AccountForCommodity::<T>::remove(&commodity_id);

        Ok(())
    }

    fn transfer(
        dest_account: &T::AccountId,
        commodity_id: &CommodityId<T>,
    ) -> dispatch::DispatchResult {
        let owner = Self::owner_of(&commodity_id);
        ensure!(
            owner != T::AccountId::default(),
            Error::<T>::NonexistentCommodity
        );

        ensure!(
            Self::total_for_account(dest_account) < T::UserCommodityLimit::get(),
            Error::<T>::TooManyCommoditiesForAccount
        );

        TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);
        TotalForAccount::<T>::mutate(dest_account, |total| *total += 1);
        let commodity = CommoditiesForAccount::<T>::mutate(owner, |commodities| {
            let pos = commodities
                .binary_search_by(|probe| probe.0.cmp(commodity_id))
                .expect("We already checked that we have the correct owner; qed");
            commodities.remove(pos)
        });
        CommoditiesForAccount::<T>::mutate(dest_account, |commodities| {
            match commodities.binary_search(&commodity) {
                Ok(_pos) => {} // should never happen
                Err(pos) => commodities.insert(pos, commodity),
            }
        });
        AccountForCommodity::<T>::insert(&commodity_id, &dest_account);

        Ok(())
    }
}
