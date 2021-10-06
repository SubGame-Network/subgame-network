//! # Lease Interface
//!
//!  A list of leasable modules is defined in pallet-lease, and users can obtain module usage rights by staking nft tokens.
//!

use sp_runtime::traits::{Member};

use sp_std::{vec::Vec};

use codec::{HasCompact};

use frame_support::{
    dispatch::{DispatchResult, DispatchError},
    Parameter,
};
pub trait Lease<AccountId, NftId> {
    type PalletId: Member + Parameter + Default + Copy + HasCompact;
    type PalletInfo;
    // type NftId;

    fn set_authority(nft_id: NftId, pallet_id: Self::PalletId, target: AccountId) -> DispatchResult;
    fn check_authority(pallet_id: Self::PalletId, target: AccountId) -> Result<bool, DispatchError>;
    fn revoke(nft_id: NftId, pallet_id: Self::PalletId) -> DispatchResult;
    fn pallet_list() -> Vec<Self::PalletInfo>;
    
}
