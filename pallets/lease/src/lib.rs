//! A list of leasable modules is defined in pallet-lease, and users can obtain module usage rights by staking nft tokens.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{Get},
    dispatch::{DispatchResult},
    Parameter,
};
use frame_system::{ensure_signed};
use sp_runtime::traits::{Member};
use sp_std::{cmp::{Eq, Ordering}, vec::Vec};

use codec::{Encode, Decode, HasCompact};

use pallet_nft;
use pallet_nft::UniqueAssets;

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
        PalletIdByNft get(fn get_pallet_id_by_nft): map hasher(blake2_128_concat) NftId<T> => T::PalletId;
        AccountByNft get(fn get_account_by_nft): map hasher(blake2_128_concat) NftId<T> => T::AccountId;

		pub NftsInfoByAccount get(fn nfts_info_by_account): double_map
        hasher(blake2_128_concat) T::AccountId,
        hasher(blake2_128_concat) T::PalletId
        => NftId<T>;
        // LeaseInfos get(fn lease_infos): map hasher(identity) NftId<T> => LeaseInfo<T::PalletId, NftId<T>>;
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
        NftIdExist,
        NotLeaseNft
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
    
        // #[weight = 10_000]
        // fn revoke(origin, nft_id: NftId<T>, pallet_id: T::PalletId) -> dispatch::DispatchResult {
        //     let sender = ensure_signed(origin)?;
        //     let admin = T::OwnerAddress::get();
        //     ensure!(admin == sender, Error::<T>::PermissionDenied);
        //     <Self as Lease<_,_>>::revoke(nft_id, pallet_id)?;
        //     Ok(())
            
        // }

        
        #[weight = 10_000]
        pub fn set_nft(origin, nft_id: NftId<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let nft_owner = T::UniqueAssets::owner_of(&nft_id);
            // need nft owner
            ensure!(nft_owner == sender, Error::<T>::PalletPermissionDenied);

            // need lease nft
            ensure!(AccountByNft::<T>::contains_key(&nft_id), Error::<T>::NotLeaseNft);

            let old_owner = AccountByNft::<T>::get(&nft_id);
            let pallet_id = PalletIdByNft::<T>::get(&nft_id);
            ensure!(old_owner != nft_owner, Error::<T>::AlreadyPallet);

            ensure!(!NftsInfoByAccount::<T>::contains_key(&nft_owner, pallet_id), Error::<T>::AlreadyPallet);
            
            NftsInfoByAccount::<T>::remove(old_owner, pallet_id);
          
            // nft_owner not exist this pallet nft
            NftsInfoByAccount::<T>::try_mutate(&nft_owner, pallet_id, |_nft_id|  -> DispatchResult {  
                *_nft_id = nft_id;
                Ok(())
            })?;
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
        ensure!(!PalletIdByNft::<T>::contains_key(&nft_id), Error::<T>::NftIdExist);
        ensure!(!NftsInfoByAccount::<T>::contains_key(&target, pallet_id), Error::<T>::NftIdExist);

        // set 
        PalletIdByNft::<T>::insert(&nft_id, &pallet_id);
        
        NftsInfoByAccount::<T>::try_mutate(&target, pallet_id, |_nft_id|  -> DispatchResult {  
            *_nft_id = nft_id.clone();
            Ok(())
        })?;
        AccountByNft::<T>::try_mutate(&nft_id, |_owner|  -> DispatchResult {  
            *_owner = target.clone();
            Ok(())
        })?;

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
        // let mut _pallets_list = Pallets::<T>::get();
        // let _pallet = _pallets_list.iter().find(|&probe| probe.pallet_id == pallet_id);
        // ensure!(_pallet != None, Error::<T>::NotFoundPallet);

        let mut have_pallet_permission = false;
        if NftsInfoByAccount::<T>::contains_key(&target, pallet_id) {
            let nft_id = NftsInfoByAccount::<T>::get(&target, pallet_id);
            // check owner
            let nft_owner = T::UniqueAssets::owner_of(&nft_id);
            if nft_owner == target {
                have_pallet_permission = true
            }else{
                // edit owner
                have_pallet_permission = false
            }
        }

        Ok(have_pallet_permission)
    }

    
    fn revoke(nft_id: NftId<T>, pallet_id: T::PalletId) -> dispatch::DispatchResult {
        // remove
        let nft_owner = T::UniqueAssets::owner_of(&nft_id);
        PalletIdByNft::<T>::remove(&nft_id);
        AccountByNft::<T>::remove(&nft_id);
        NftsInfoByAccount::<T>::remove(nft_owner, pallet_id);
        Ok(())
    }
}
