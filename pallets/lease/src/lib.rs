//! A list of leasable modules is defined in pallet-lease, and users can obtain module usage rights by staking nft tokens.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{Get},
    Parameter,
};
use frame_system::ensure_signed;
use sp_runtime::traits::{Member};
use sp_std::{cmp::{Eq, Ordering}, fmt::Debug, vec::Vec};

use codec::{Encode, Decode, HasCompact};

use pallet_nft;
use pallet_nft::UniqueAssets;

use frame_support::{
    debug,
};

pub mod lease;
pub use crate::lease::Lease;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, Eq)]
/// Contains the pallet name and description, you can easily understand the pallet information.
pub struct PalletInfo<PalletId> {
    pallet_id: PalletId,
    name: Vec<u8>,
}

impl<PalletId: Ord> Ord for PalletInfo<PalletId> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pallet_id.cmp(&other.pallet_id)
    }
}

impl<PalletId: Ord> PartialOrd for PalletInfo<PalletId> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.pallet_id.cmp(&other.pallet_id))
    }
}

impl<PalletId: Eq> PartialEq for PalletInfo<PalletId> {
    fn eq(&self, other: &Self) -> bool {
        self.pallet_id == other.pallet_id
    }
}


#[derive(Encode, Decode, Debug, Default, Copy, Clone, Eq)]
/// Record lease information.
pub struct LeaseInfo<PalletId, NftId> {
    pub pallet_id: PalletId,
    pub nft_id: NftId,
}

impl<PalletId: Ord, NftId: Eq> Ord for LeaseInfo<PalletId, NftId> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pallet_id.cmp(&other.pallet_id)
    }
}

impl<PalletId: Ord, NftId> PartialOrd for LeaseInfo<PalletId, NftId> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.pallet_id.cmp(&other.pallet_id))
    }
}

impl<PalletId: Eq, NftId> PartialEq for LeaseInfo<PalletId, NftId> {
    fn eq(&self, other: &Self) -> bool {
        self.pallet_id == other.pallet_id
    }
}



pub trait Config: frame_system::Config {
    /// The owner can manage the pallet list and set permissions.
    type OwnerAddress: Get<Self::AccountId>;
    /// The data type that is used to describe this type of NFT.
    type UniqueAssets: UniqueAssets<Self::AccountId>;
    /// Indicates the id type of the pallet
    type PalletId: Member + Parameter + Default + Copy + HasCompact + Ord;
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

pub type NftId<T> = 
    <<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetId;

decl_storage! {
    trait Store for Module<T: Config> as Lease {
        /// save can lease pallet list
        Pallets get(fn pallets): Vec<PalletInfo<T::PalletId>>;
        // NftToPalletId get(fn nft_to_pallet_id): map hasher(blake2_128_concat) NftId<T> => u64;
        LeaseInfos get(fn lease_infos): map hasher(identity) NftId<T> => LeaseInfo<T::PalletId, NftId<T>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        // StakeId = <T as frame_system::Config>::Hash,
        NftId = NftId<T>,
        AccountId = <T as frame_system::Config>::AccountId,
        PalletId = <T as Config>::PalletId,
    {
        NewPallet(PalletId, Vec<u8>),
        RemovePallet(PalletId),
        SetAuthority(AccountId, NftId, PalletId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        NotFoundPallet,
        AlreadyPallet,
        PermissionDenied,
        PalletPermissionDenied,
        NftIdExist
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;
        
        #[weight = 10_000]
        pub fn add_pallet(origin, pallet_id: T::PalletId, name: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let admin = T::OwnerAddress::get();
            ensure!(admin == sender, Error::<T>::PermissionDenied);
            
            let mut _pallets = Pallets::<T>::get();

            let new_pallet = PalletInfo { 
                pallet_id: pallet_id, 
                name: name.clone() 
            };
            
            match _pallets.binary_search(&new_pallet) {
                // If the search succeeds, the caller is already have pallet, so just return
                Ok(_) => Err(Error::<T>::AlreadyPallet.into()),
                // If the search fails, the caller is not have pallet and we learned the index where
                // they should be inserted
                Err(index) => {
                    _pallets.insert(index, new_pallet.clone());
                    Pallets::<T>::put(_pallets);
                    Self::deposit_event(RawEvent::NewPallet(pallet_id, name));
                    Ok(())
                }
            }
        }

        #[weight = 10_000]
        pub fn del_pallet(origin, pallet_id: T::PalletId) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let admin = T::OwnerAddress::get();
            ensure!(admin == sender, Error::<T>::PermissionDenied);
            
            let mut _pallets = Pallets::<T>::get();

            match _pallets.binary_search_by(|probe| probe.pallet_id.cmp(&pallet_id)){
                Ok(index) => {
                    _pallets.remove(index);
                    Pallets::<T>::put(_pallets);
                    Self::deposit_event(RawEvent::RemovePallet(pallet_id));
                    Ok(())
                }
                Err(_) => Err(Error::<T>::NotFoundPallet.into()),
            }
        }
        
        #[weight = 10_000]
        pub fn set_authority(origin, nft_id: NftId<T>, pallet_id: T::PalletId, target: T::AccountId) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let admin = T::OwnerAddress::get();
            ensure!(admin == sender, Error::<T>::PermissionDenied);
            <Self as Lease<_,_>>::set_authority(nft_id, pallet_id, target)?;

            Ok(())
        }

        #[weight = 10_000]
        fn check_authority(origin, pallet_id: T::PalletId) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let have_pallet_permission = <Self as Lease<_,_>>::check_authority(pallet_id, sender)?;
            
            ensure!(have_pallet_permission, Error::<T>::PalletPermissionDenied);
            Ok(())
        }
    
        #[weight = 10_000]
        fn revoke(origin, nft_id: NftId<T>, pallet_id: T::PalletId) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let admin = T::OwnerAddress::get();
            ensure!(admin == sender, Error::<T>::PermissionDenied);
            <Self as Lease<_,_>>::revoke(nft_id, pallet_id)?;
            Ok(())
            
        }
        
    }
}

impl<T: Config> Lease<T::AccountId, NftId<T>> for Module<T> {
    type PalletId = T::PalletId;
    type PalletInfo = PalletInfo<T::PalletId>;
    // type NftId = NftId<T>;
    
    fn pallet_list() -> Vec<PalletInfo<T::PalletId>> {
        Self::pallets()
    }

    fn set_authority(nft_id: NftId<T>, pallet_id: T::PalletId, target: T::AccountId) -> dispatch::DispatchResult {

        // check pallet exist
        let mut _pallets_list = Pallets::<T>::get();
        let _pallet = _pallets_list.iter().find(|&probe| probe.pallet_id == pallet_id);
        ensure!(_pallet != None, Error::<T>::NotFoundPallet);

        // check nft owner is match
        let nft_owner = T::UniqueAssets::owner_of(&nft_id);
        ensure!(nft_owner == target, Error::<T>::PalletPermissionDenied);
        
        // check nft_id exist
        let lease_info = LeaseInfos::<T>::get(&nft_id);
        debug::info!("lease_info ：{:?}", lease_info);
        debug::info!("lease_info.pallet_id ：{:?}", lease_info.pallet_id);

        ensure!(lease_info.nft_id != nft_id, Error::<T>::NftIdExist);

        // set 
        LeaseInfos::<T>::insert(&nft_id, LeaseInfo{
            pallet_id: pallet_id,
            nft_id: nft_id.clone(),
        });
        // Notification of create game
        Self::deposit_event(RawEvent::SetAuthority(
            target, 
            nft_id.clone(), 
            pallet_id
        ));

        Ok(())
    }


    fn check_authority(pallet_id: T::PalletId, target: T::AccountId) -> dispatch::result::Result<bool, dispatch::DispatchError> {
        // check pallet exist
        let mut _pallets_list = Pallets::<T>::get();
        let _pallet = _pallets_list.iter().find(|&probe| probe.pallet_id == pallet_id);
        ensure!(_pallet != None, Error::<T>::NotFoundPallet);


        // Find out all nfts owned by the target
        let assets = T::UniqueAssets::assets_for_account(&target);
    
        // Determine whether Nft_id has the right to use pallet_id
        let mut have_pallet_permission = false;
        for _asset in assets.iter() {
            let _nft_id = _asset.0.clone();
            if LeaseInfos::<T>::contains_key(_nft_id.clone()) {
                let lease_info = LeaseInfos::<T>::get(_nft_id);
                if lease_info.pallet_id == pallet_id {
                    have_pallet_permission = true;
                    break;
                }
            }
           
        }
        Ok(have_pallet_permission)
    }

    
    fn revoke(nft_id: NftId<T>, pallet_id: T::PalletId) -> dispatch::DispatchResult {
        // check pallet exist
        let mut _pallets_list = Pallets::<T>::get();
        let _pallet = _pallets_list.iter().find(|&probe| probe.pallet_id == pallet_id);
        ensure!(_pallet != None, Error::<T>::NotFoundPallet);

        // remove
        LeaseInfos::<T>::remove(&nft_id);

        Ok(())
    }
}
