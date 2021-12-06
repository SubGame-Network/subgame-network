//! After providing users to stake SGB, they can get an nft token, and they can use special functions with nft token. SGB will be returned through redemption, and nft token will be burned at the same time. The module will provide different stake amount schemes and different valid periods. When the stake expires, nft token can no longer be used for special functions, SGB can be returned through redemption, and nft token will be burned.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{Currency, ExistenceRequirement, Get},
    Parameter,
};
use frame_system::ensure_signed;
use sp_runtime::traits::{Member};
use sp_std::{cmp::{Eq, Ordering}, vec::Vec};

use codec::{Encode, Decode, HasCompact};
use codec::{alloc::string::{ToString}};

use pallet_nft::UniqueAssets;
use pallet_lease::Lease;

use frame_support::{
    debug,
};

use frame_support::sp_std::convert::{TryInto, TryFrom};

pub mod stake_nft;
pub use crate::stake_nft::StakeNft;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Copy, Clone, Eq)]
pub struct Program<ProgramId, Balance> {
    program_id: ProgramId,
    stake_amount: Balance,
    valid_day_count: u64,
}

impl<ProgramId: Ord, Balance: Eq> Ord for Program<ProgramId, Balance> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.program_id.cmp(&other.program_id)
    }
}

impl<ProgramId: Ord, Balance> PartialOrd for Program<ProgramId, Balance> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.program_id.cmp(&other.program_id))
    }
}

impl<ProgramId: Eq, Balance> PartialEq for Program<ProgramId, Balance> {
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
    }
}

#[derive(Encode, Decode, Default, Copy, Clone, Eq)]
pub struct StakeInfo<ProgramId, PalletId, Balance, NftId> {
    program_id: ProgramId,
    pallet_id: PalletId,
    stake_amount: Balance,
    will_expire: bool,
    expires_at: i64,
    nft_id: NftId,
}

impl<ProgramId: Ord, PalletId: Eq, NftId: Eq, Balance: Eq> Ord for StakeInfo<ProgramId, PalletId, Balance, NftId> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.program_id.cmp(&other.program_id)
    }
}

impl<ProgramId: Ord, PalletId, Balance, NftId> PartialOrd for StakeInfo<ProgramId, PalletId, Balance, NftId> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.program_id.cmp(&other.program_id))
    }
}

impl<ProgramId: Eq, PalletId, Balance, NftId> PartialEq for StakeInfo<ProgramId, PalletId, Balance, NftId> {
    fn eq(&self, other: &Self) -> bool {
        self.program_id == other.program_id
    }
}



pub trait Config: frame_system::Config + pallet_timestamp::Config {
    /// The dispatch origin that is able to mint new instances of this type of commodity.
    type OwnerAddress: Get<Self::AccountId>;
    /// The data type that is used to describe this type of commodity.
    type UniqueAssets: UniqueAssets<Self::AccountId>;
    type Lease: Lease<Self::AccountId, NftId<Self>>;

    type ProgramId: Member + Parameter + Default + Copy + HasCompact + Ord;
    type PalletId: Member + Parameter + Default + Copy + HasCompact + Ord;
    type Balances: Currency<Self::AccountId>;
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

/// The runtime system's hashing algorithm is used to uniquely identify commodities.
pub type StakeId<T> = <T as frame_system::Config>::Hash;
pub type UniqueAssetInfoOf<T> = <<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetInfo;

pub type BalanceOf<T> =
    <<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type NftId<T> = 
<<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetId;
pub type PalletId<T> = 
    <<T as Config>::Lease as Lease<<T as frame_system::Config>::AccountId, NftId<T>>>::PalletId;
pub type MomentOf<T> = <T as pallet_timestamp::Config>::Moment;

decl_storage! {
    trait Store for Module<T: Config> as StakeNFT {
        Programs get(fn programs_list): Vec<Program<T::ProgramId, BalanceOf<T>>>;
        StakeUsers get(fn stake_users):  Vec<T::AccountId>;
        StakeInfos get(fn stake_infos): map hasher(blake2_128_concat) T::AccountId => Vec<StakeInfo<T::ProgramId, PalletId<T>, BalanceOf<T>, NftId<T>>>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        // StakeId = <T as frame_system::Config>::Hash,
        NftId = NftId<T>,
        AccountId = <T as frame_system::Config>::AccountId,
        ProgramId = <T as Config>::ProgramId,
        PalletId = PalletId<T>,
        BalanceOf = BalanceOf<T>
    {
        ProgramAdded(ProgramId, BalanceOf, u64),
        Stake(AccountId, ProgramId, PalletId, u64, Vec<u8>, Vec<u8>, NftId, BalanceOf),
        UpdateStakeExpire(AccountId, NftId, bool),
        Renew(AccountId, NftId, Vec<u8>),
        Expire(AccountId, NftId),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        AlreadyProgram,
        NotFoundProgram,
        AlreadyPallet,
        NotFoundPallet,
        AlreadyStake,
        NotFoundNft,
        MoneyNotEnough,
        PermissionDenied,
        NotFoundData
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;
        
        #[weight = 10_000]
        pub fn add_program(origin, program_id: T::ProgramId, stake_amount: BalanceOf<T>, day: u64) -> dispatch::DispatchResult {            
            let sender = ensure_signed(origin)?;
            let admin = T::OwnerAddress::get();
            ensure!(admin == sender, Error::<T>::PermissionDenied);
            
            let mut _programs_list = Programs::<T>::get();

            let new_program = Program { 
                program_id: program_id, 
                stake_amount: stake_amount, 
                valid_day_count: day 
            };
            
            match _programs_list.binary_search(&new_program) {
                Ok(_) => Err(Error::<T>::AlreadyProgram.into()),
                Err(index) => {
                    _programs_list.insert(index, new_program.clone());
                    Programs::<T>::put(_programs_list);
                    Self::deposit_event(RawEvent::ProgramAdded(program_id, stake_amount, day));
                    Ok(())
                }
            }
        }

        #[weight = 10_000]
        pub fn del_program(origin, program_id: T::ProgramId) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let admin = T::OwnerAddress::get();
            ensure!(admin == sender, Error::<T>::PermissionDenied);
            
            let mut _programs_list = Programs::<T>::get();

            match _programs_list.binary_search_by(|probe| probe.program_id.cmp(&program_id)){
                Ok(index) => {
                    _programs_list.remove(index);
                    Programs::<T>::put(_programs_list);
                    Ok(())
                }
                Err(_) => Err(Error::<T>::NotFoundProgram.into()),
            }
        }
      
        #[weight = 10_000]
        pub fn stake(origin, program_id: T::ProgramId, pallet_id: PalletId<T>) -> dispatch::DispatchResult {
            let from_address = ensure_signed(origin)?;

            // check program exist
            let mut _programs_list = Programs::<T>::get();
            let _program = _programs_list.iter().find(|&&probe| probe.program_id == program_id);
            ensure!(_program != None, Error::<T>::NotFoundProgram);

            ensure!(T::Balances::free_balance(&from_address) >= _program.unwrap().stake_amount.into(), Error::<T>::MoneyNotEnough);

            // check stake exist
            let stake_list = StakeInfos::<T>::get(from_address.clone());
            let _stake = stake_list.iter().find(|&probe| probe.pallet_id == pallet_id);
            ensure!(_stake == None, Error::<T>::AlreadyStake);


            let commodity_id = T::UniqueAssets::mint(&from_address.clone(), Vec::new())?;
            let owner = T::OwnerAddress::get();


            // now time
            let now = pallet_timestamp::Pallet::<T>::get();
            let now_ms = TryInto::<u64>::try_into(now).ok().unwrap(); // convert to u64

            // add N day
            let n_day = _program.unwrap().valid_day_count as i64;
            let n_day_ms = u64::try_from(chrono::Duration::days(n_day).num_milliseconds()).ok().unwrap();
            let expires_at_ms = now_ms + n_day_ms;

            let now_timestamp = now_ms / 1000;
            let expires_at = expires_at_ms / 1000;

            let new_stake_nft = StakeInfo{
                pallet_id: pallet_id,
                program_id: program_id,
                stake_amount: _program.unwrap().stake_amount,
                will_expire: false,
                expires_at: expires_at as i64,
                nft_id: commodity_id.clone(),
            };

            T::Lease::set_authority(commodity_id.clone(), pallet_id, from_address.clone())?;
            T::Balances::transfer(&from_address, &owner, _program.unwrap().stake_amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough)?;
            StakeInfos::<T>::mutate(from_address.clone(), |stake_nft_data| {
                stake_nft_data.insert(stake_nft_data.len(), new_stake_nft.clone())
            });
            
            let mut users = StakeUsers::<T>::get();
            match users.binary_search(&from_address) {
                // If the search succeeds, the caller is already a member, so just return
                Ok(_) => {},
                // If the search fails, the caller is not a member and we learned the index where
                // they should be inserted
                Err(index) => {
                    users.insert(index, from_address.clone());
                    StakeUsers::<T>::put(users);
                }
            } 

            Self::deposit_event(RawEvent::Stake(from_address, program_id, pallet_id, _program.unwrap().valid_day_count, now_timestamp.to_string().into_bytes(), expires_at.to_string().into_bytes(), commodity_id.clone(), _program.unwrap().stake_amount));        
            Ok(())
        }
      
        #[weight = 10_000]
        pub fn set_stake_will_expire(origin, nft_id: NftId<T>, will_expire: bool) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;

            // check stake exist
            let stake_list = StakeInfos::<T>::get(sender.clone());
            let index_option = stake_list.iter().position(|probe| probe.nft_id == nft_id);
            ensure!(index_option != None, Error::<T>::NotFoundNft);
            let index = index_option.unwrap();

            Self::deposit_event(RawEvent::UpdateStakeExpire(sender.clone(), nft_id.clone(), will_expire));  
            StakeInfos::<T>::try_mutate_exists(sender, |stake_info| {
                let _stake_info = stake_info.as_mut().ok_or( Error::<T>::NotFoundData)?;
                _stake_info[index].will_expire = will_expire;
                Ok(())
            })
        }
        
        fn on_finalize() {
            let now = pallet_timestamp::Pallet::<T>::get();
            let now_timestamp = (TryInto::<u64>::try_into(now).ok().unwrap() / 1000) as i64; 
            let users = StakeUsers::<T>::get();
            for user in users {
                let stakes = StakeInfos::<T>::get(user.clone());
                
                for (index, stake) in stakes.iter().enumerate() {
                    // expires
                    if now_timestamp > stake.expires_at {
                        // 過期不自動續約
                        if stake.will_expire {
                             // nft_owner
                            // let nft_owner = T::UniqueAssets::owner_of(&stake.nft_id.clone());

                            let owner = T::OwnerAddress::get();
                            // check balance
                            if T::Balances::free_balance(&owner) <= stake.stake_amount {
                                debug::info!("stake-nft owner餘額不足，無法進行退款, nft: {:?}", stake.nft_id.clone());
                                return
                            }

                            debug::info!("stake-nft 過期,已註銷, nft: {:?}", stake.nft_id.clone());

                            T::UniqueAssets::burn(&stake.nft_id.clone()).map_err(|err| debug::error!("err: {:?}", err)).ok();

                            T::Lease::revoke(stake.nft_id.clone(), stake.pallet_id).map_err(|err| debug::error!("err: {:?}", err)).ok();

                            T::Balances::transfer(&owner, &user, stake.stake_amount, ExistenceRequirement::KeepAlive).map_err(|err| debug::error!("err: {:?}", err)).ok();
                
                            // remove record
                            StakeInfos::<T>::mutate(user.clone(), |stake_nft_data| {
                                stake_nft_data.remove(index);
                            });

                            Self::deposit_event(RawEvent::Expire(user.clone(), stake.nft_id.clone()));        
                            
                        }else{
                            debug::info!("stake-nft 過期,自動續約, nft: {:?}", stake.nft_id);

                            let mut _programs_list = Programs::<T>::get();
                            let _program = _programs_list.iter().find(|&&probe| probe.program_id == stake.program_id);
                            // now time
                            let now = pallet_timestamp::Pallet::<T>::get();
                            let now_ms = TryInto::<u64>::try_into(now).ok().unwrap(); // convert to u64

                            // add N day
                            let n_day = _program.unwrap().valid_day_count as i64;
                            let n_day_ms = u64::try_from(chrono::Duration::days(n_day).num_milliseconds()).ok().unwrap();
                            let expires_at_ms = now_ms + n_day_ms;

                            let expires_at = expires_at_ms / 1000;

                            StakeInfos::<T>::mutate(user.clone(), |stake_info| {
                                stake_info[index].expires_at = expires_at as i64;
                            });
                            Self::deposit_event(RawEvent::Renew(user.clone(), stake.nft_id.clone(), expires_at.to_string().into_bytes()));  
                        }
                       
                    }
                } 
            }
           
           
        }
    }
}
