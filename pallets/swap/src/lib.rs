//! Responsible for managing the user’s chips, after purchasing chips, you can use the chips to participate in the game

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{Currency, ExistenceRequirement, Get},
    weights::Weight,
};
use frame_system::ensure_signed;

use frame_support::traits::Vec;

mod default_weight;
pub trait WeightInfo {
    fn send() -> Weight;
    fn receive_swap() -> Weight;
}

#[derive(Encode, Decode, Default, Copy, Clone)]
pub struct SwapRecord<Account1, Account2, ChainType, CoinType> {
    from: Account1,
    to: Account2,
    amount: u128,
    chain_type: ChainType, 
    coin_type: CoinType,
}

/// Define the chain type
pub type ChainType = u8;
/// subgame
pub const CHAIN_SUBGAME: ChainType = 1;
/// eth
pub const CHAIN_ETH: ChainType = 2;
/// heco
pub const CHAIN_HECO: ChainType = 3;

/// Define the coin type
pub type CoinType = u8;
/// sgb
pub const COIN_SGB: CoinType = 1;



pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Balances: Currency<Self::AccountId>;
    /// The address where funds are temporarily deposited
    type OwnerAddress: Get<Self::AccountId>;
    type WeightInfo: WeightInfo;
}

pub type BalanceOf<T> =
    <<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;
decl_storage! {
    trait Store for Module<T: Config> as Chips {
        /// from other chain to subgame chain
        pub InRecord get(fn in_record):Vec<SwapRecord<Vec<u16>, T::AccountId, ChainType, CoinType> >;
        /// subgame chain to other chain
        pub OutRecord get(fn out_record):Vec<SwapRecord<T::AccountId, Vec<u16>, ChainType, CoinType> >;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        BalanceOf = BalanceOf<T>
    {
        /// Swap to subgame
        Send(AccountId, BalanceOf, Vec<u16>),
        /// Swap from subgame
        ReceiveSwap(AccountId, Vec<u16>, ChainType, CoinType, BalanceOf),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
        StorageOverflow,
        MoneyNotEnough,
        CoinTypeNotFound,
        ChainTypeNotFound,
        NeverBoughtChips,
        PermissionDenied
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        type Error = Error<T>;
        fn deposit_event() = default;

        /// outchain to subgame (sgb)
        #[weight = T::WeightInfo::send()]
        pub fn send(origin, to_address: T::AccountId, amount: BalanceOf<T>, coin_type: CoinType, hash: Vec<u16>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            ensure!(owner == sender, Error::<T>::PermissionDenied);
           
            ensure!(coin_type == COIN_SGB, Error::<T>::CoinTypeNotFound);
        
            T::Balances::transfer(&owner, &to_address, amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough)?;

            // Send event notification
            Self::deposit_event(RawEvent::Send(to_address, amount, hash));
            Ok(())
        }

        
        /// outchain to subgame (sgb)
        #[weight = T::WeightInfo::receive_swap()]
        pub fn receive_swap(origin, to_address: Vec<u16>, amount: BalanceOf<T>, chain_type: ChainType, coin_type: CoinType) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let owner = T::OwnerAddress::get();
            
            ensure!(chain_type == CHAIN_ETH || chain_type == CHAIN_HECO, Error::<T>::ChainTypeNotFound);
            ensure!(coin_type == COIN_SGB, Error::<T>::CoinTypeNotFound);

            T::Balances::transfer(&sender, &owner, amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough)?;

            // Send event notification
            Self::deposit_event(RawEvent::ReceiveSwap(sender, to_address, chain_type, coin_type, amount));
            Ok(())
        }

    }
}
