//! Responsible for managing the user’s chips, after purchasing chips, you can use the chips to participate in the game

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{Currency, ExistenceRequirement, Get},
    weights::{Weight},
    debug,

};
use sp_std::convert::TryInto;

use pallet_subgame_assets::{AssetsTrait, AssetsTransfer};
use frame_system::ensure_signed;
// use sp_runtime::{{
//     traits::{CheckedAdd, CheckedSub}
// }};
use frame_support::traits::Vec;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod default_weight;
pub trait WeightInfo {
    fn send() -> Weight;
    fn receive_bridge() -> Weight;
    fn update_min_limit() -> Weight;
}

#[derive(Encode, Decode, Default, Copy, Clone)]
pub struct BridgeRecord<Account1, Account2, ChainType, CoinType> {
    from: Account1,
    to: Account2,
    amount: u128,
    chain_type: ChainType, 
    coin_type: CoinType,
}
/// Define the chain type
/// subgame
pub const CHAIN_SUBGAME: u8 = 1;
/// eth
pub const CHAIN_ETH: u8 = 2;
/// heco
pub const CHAIN_HECO: u8 = 3;
pub const CHAIN_BSC: u8 = 4;
pub const CHAIN_OKC: u8 = 5;
pub const CHAIN_TRON: u8 = 6;

/// Define the coin type
/// sgb
pub const COIN_SGB: u8 = 1;
/// usdt
pub const COIN_USDT: u8 = 7;

pub const SGB_BALANCE_UNIT: u64 = 10000000000 ;


pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Balances: Currency<Self::AccountId>;
    /// The address where funds are temporarily deposited
    type OwnerAddress: Get<Self::AccountId>;
    type WeightInfo: WeightInfo;

    type Assets: AssetsTrait + AssetsTransfer<Self::AccountId, u32>;
}

pub type BalanceOf<T> =
    <<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;
decl_storage! {
    trait Store for Module<T: Config> as Chips {
        /// from other chain to subgame chain
        pub InRecord get(fn in_record):Vec<BridgeRecord<Vec<u8>, T::AccountId, u8, u8> >;
        /// subgame chain to other chain
        pub OutRecord get(fn out_record):Vec<BridgeRecord<T::AccountId, Vec<u8>, u8, u8> >;
        // bridge amount need bigger than BridgeMinLimit
        pub BridgeMinLimit get(fn bridge_min_limit): Option<BalanceOf<T>>;

    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        BalanceOf = BalanceOf<T>
    {
        /// Bridge to subgame
        Send(AccountId, BalanceOf, Vec<u8>),
        /// Bridge from subgame
        ReceiveBridge(AccountId, Vec<u8>, u8, u8, BalanceOf),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        StorageOverflow,
        MoneyNotEnough,
        CoinTypeNotFound,
        ChainTypeNotFound,
        AssetAmountDenied,
        NeverBoughtChips,
        PermissionDenied,
        BridgeNotEnoughMinLimt,
        SwapAmountLessThenLimit
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// outchain to subgame (sgb)
        #[weight = T::WeightInfo::send()]
        pub fn send(origin, to_address: T::AccountId, amount: BalanceOf<T>, coin_type: u8, hash: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            ensure!(owner == sender, Error::<T>::PermissionDenied);
            debug::info!("mint log：{:?}", amount);
           
            ensure!(coin_type == COIN_SGB || coin_type == COIN_USDT, Error::<T>::CoinTypeNotFound);
            // ensure!(coin_type == COIN_SGB, Error::<T>::CoinTypeNotFound);

            if coin_type == COIN_SGB {
                T::Balances::transfer(&owner, &to_address, amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough)?;
            }else{
                let asset_amount = TryInto::<u64>::try_into(amount).ok();
                // 確認是否成功轉換
                ensure!(asset_amount != None, Error::<T>::AssetAmountDenied);

                T::Assets::mint(owner.clone(), coin_type.into(), to_address.clone(), asset_amount.unwrap())?;
                // debug::info!("mint log：{:?}", result);
            }

            // Send event notification
            Self::deposit_event(RawEvent::Send(to_address, amount, hash));
            Ok(())
        }

        
        /// outchain to subgame (sgb)
        #[weight = T::WeightInfo::receive_bridge()]
        pub fn receive_bridge(origin, to_address: Vec<u8>, amount: BalanceOf<T>, chain_type: u8, coin_type: u8) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            let default_limit: BalanceOf<T> = 10u32.into();
            // let bridge_min_limit = Self::bridge_min_limit();
            
            ensure!(amount >= default_limit, Error::<T>::SwapAmountLessThenLimit);
            
           
            ensure!(
                chain_type == CHAIN_ETH || chain_type == CHAIN_HECO || chain_type == CHAIN_BSC || 
                chain_type == CHAIN_OKC || 
                chain_type == CHAIN_TRON
                , Error::<T>::ChainTypeNotFound);
            ensure!(coin_type == COIN_SGB || coin_type == COIN_USDT, Error::<T>::CoinTypeNotFound);
            // ensure!(coin_type == COIN_SGB, Error::<T>::CoinTypeNotFound);

            if coin_type == COIN_SGB {
                T::Balances::transfer(&sender, &owner, amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough)?;
            }else{
                let asset_amount = TryInto::<u64>::try_into(amount).ok();
                // 確認是否成功轉換
                ensure!(asset_amount != None, Error::<T>::AssetAmountDenied);
                
                // debug::info!("burn log：{:?}", asset_amount);
                T::Assets::burn(owner.clone(), coin_type.into(), sender.clone(), asset_amount.unwrap())?;
                // debug::info!("burn log：{:?}", result);
            }

            // Send event notification
            Self::deposit_event(RawEvent::ReceiveBridge(sender, to_address, chain_type, coin_type, amount));
            Ok(())
        }

        /// outchain to subgame (sgb)
        #[weight = T::WeightInfo::update_min_limit()]
        pub fn update_min_limit(origin, amount: BalanceOf<T>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            ensure!(owner == sender, Error::<T>::PermissionDenied);
            <BridgeMinLimit<T>>::put(amount);
            Ok(())
        }

        // /// test token deposit
        // #[weight = T::WeightInfo::receive_bridge()]
        // pub fn test_toke_deposit(origin, asset_id: u32, to_address: T::AccountId , amount: BalanceOf<T>) -> dispatch::DispatchResult {
        //     let sender = ensure_signed(origin)?;
        //     let result = T::Assets::mint(sender, asset_id, to_address.clone(), amount.try_into().ok())?;
        //     debug::info!("{:?}", result);
        //     Ok(())
        // }

        // /// test token withdraw
        // #[weight = T::WeightInfo::receive_bridge()]
        // pub fn test_toke_withdraw(origin, asset_id: u32, to_address: T::AccountId , amount: BalanceOf<T>) -> dispatch::DispatchResult {
        //     let sender = ensure_signed(origin)?;
        //     let result = T::Assets::burn(sender, asset_id, to_address.clone(), amount.try_into().ok())?;
        //     debug::info!("{:?}", result);
        //     Ok(())
        // }
    }
}
