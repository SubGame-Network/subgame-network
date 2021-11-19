//! SubGame Stake Dapp

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, 
    traits::{Get, Currency, ReservableCurrency, ExistenceRequirement, Vec},
    weights::{Weight},
};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod default_weight;

pub trait WeightInfo {
    fn sign_up() -> Weight;
    fn stake() -> Weight;
    fn unlock() -> Weight;
    fn withdraw() -> Weight;
    fn import_stake() -> Weight;
    fn delete_user() -> Weight;
}

type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Default)]
pub struct UserInfo<Account, ReferrerAccount> {
    pub account: Account,
    pub referrer_account: ReferrerAccount,
}

pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Balances: Currency<Self::AccountId>;
    type OwnerAddress: Get<Self::AccountId>;
    type ImportAddress: Get<Self::AccountId>;
    type WeightInfo: WeightInfo;
    type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}

decl_storage! {
    trait Store for Module<T: Config> as Chips {
        pub UserInfoMap get(fn user_info_map): map hasher(blake2_128_concat) T::AccountId => UserInfo<Vec<u8>, Vec<u8>>;
        pub AccountMap get(fn account_map): map hasher(blake2_128_concat) Vec<u8> => T::AccountId;
        pub UserStake get(fn user_stake): map hasher(blake2_128_concat) T::AccountId => BalanceOf<T>;
        pub StakePool get(fn stake_pool): BalanceOf<T>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        Balance = BalanceOf<T>,
    {
        SignUp(AccountId, Vec<u8>, Vec<u8>),
        Stake(AccountId, Balance),
        Unlock(AccountId, Balance),
        Withdraw(AccountId, Balance),
        DeleteUser(AccountId, Vec<u8>),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        UserExists,
        AccountFormatIsWrong,
        UserNotExists,
        MoneyNotEnough,
        PermissionDenied,
        StakeAmountWrong,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        #[weight = T::WeightInfo::sign_up()]
        pub fn sign_up(origin, account: Vec<u8>, referrer_account: Vec<u8>) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            
            let _account_str = core::str::from_utf8(&account).unwrap().to_lowercase();
            ensure!(_account_str.len() == 7, Error::<T>::AccountFormatIsWrong);
            ensure!(_account_str != "gametop", Error::<T>::AccountFormatIsWrong);
            let _account = _account_str.as_bytes().to_vec();
            
            let _referrer_account_str = core::str::from_utf8(&referrer_account).unwrap().to_lowercase();
            let _referrer_account = _referrer_account_str.as_bytes().to_vec();

            ensure!(!UserInfoMap::<T>::contains_key(&_who), Error::<T>::UserExists);
            ensure!(!AccountMap::<T>::contains_key(_account.clone()), Error::<T>::UserExists);

            let user_info = UserInfo{
                account: _account.clone(),
                referrer_account: _referrer_account.clone(),
            };
            <UserInfoMap::<T>>::insert(&_who, user_info);
            <AccountMap::<T>>::insert(_account.clone(), &_who);

            Self::deposit_event(RawEvent::SignUp(_who, _account, _referrer_account));
            Ok(())
        }

        #[weight = T::WeightInfo::stake()]
        pub fn stake(origin, amount: BalanceOf<T>) -> dispatch::DispatchResult {
            let _who = ensure_signed(origin)?;
            ensure!(UserInfoMap::<T>::contains_key(&_who), Error::<T>::UserNotExists);

            T::Currency::reserve(&_who, amount).map_err(|_| Error::<T>::MoneyNotEnough )?;
            <StakePool::<T>>::put(Self::stake_pool() + amount);
            <UserStake::<T>>::insert(&_who, Self::user_stake(&_who) + amount);

            Self::deposit_event(RawEvent::Stake(_who, amount));
            Ok(())
        }

        #[weight = T::WeightInfo::unlock()]
        pub fn unlock(origin, _who: T::AccountId, amount: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            ensure!(owner == sender, Error::<T>::PermissionDenied);
            ensure!(UserInfoMap::<T>::contains_key(&_who), Error::<T>::UserNotExists);
            ensure!(T::Currency::reserved_balance(&_who) >= amount, Error::<T>::MoneyNotEnough);

            let user_stake = Self::user_stake(&_who);
            ensure!(user_stake >= amount, Error::<T>::MoneyNotEnough);
            
            let stake_pool = Self::stake_pool();
            ensure!(stake_pool >= amount, Error::<T>::MoneyNotEnough);

            T::Currency::unreserve(&_who, amount);
            <StakePool::<T>>::put(stake_pool - amount);
            <UserStake::<T>>::insert(&_who, user_stake - amount);

            Self::deposit_event(RawEvent::Unlock(_who, amount));
            Ok(())
        }

        #[weight = T::WeightInfo::withdraw()]
        pub fn withdraw(origin, _who: T::AccountId, amount: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            ensure!(owner == sender, Error::<T>::PermissionDenied);
            ensure!(UserInfoMap::<T>::contains_key(&_who), Error::<T>::UserNotExists);
            ensure!(T::Currency::free_balance(&owner) >= amount, Error::<T>::MoneyNotEnough);

            T::Currency::transfer(&sender, &_who, amount, ExistenceRequirement::KeepAlive)?;

            Self::deposit_event(RawEvent::Withdraw(_who, amount));
            Ok(())
        }

        #[weight = T::WeightInfo::import_stake()]
        pub fn import_stake(origin, _who: T::AccountId, amount: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            let import_owner = T::ImportAddress::get();
            ensure!(owner == sender, Error::<T>::PermissionDenied);
            ensure!(UserInfoMap::<T>::contains_key(&_who), Error::<T>::UserNotExists);

            T::Currency::transfer(&import_owner, &_who, amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough )?;

            T::Currency::reserve(&_who, amount).map_err(|_| Error::<T>::MoneyNotEnough )?;
            <StakePool::<T>>::put(Self::stake_pool() + amount);
            <UserStake::<T>>::insert(&_who, Self::user_stake(&_who) + amount);

            Self::deposit_event(RawEvent::Stake(_who, amount));
            Ok(())
        }

        #[weight = T::WeightInfo::delete_user()]
        pub fn delete_user(origin, _who: T::AccountId, account: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            ensure!(owner == sender, Error::<T>::PermissionDenied);
            ensure!(UserInfoMap::<T>::contains_key(&_who), Error::<T>::UserNotExists);

            let _account_str = core::str::from_utf8(&account).unwrap().to_lowercase();
            ensure!(_account_str.len() == 7, Error::<T>::AccountFormatIsWrong);
            ensure!(_account_str != "gametop", Error::<T>::AccountFormatIsWrong);
            let _account = _account_str.as_bytes().to_vec();

            <UserInfoMap::<T>>::remove(&_who);
            <AccountMap::<T>>::remove(_account.clone());

            Self::deposit_event(RawEvent::DeleteUser(_who, account));
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {

}