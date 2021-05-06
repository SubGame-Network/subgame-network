//! Responsible for managing the user’s chips, after purchasing chips, you can use the chips to participate in the game

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
	weights::Weight, 
	Parameter,
	traits::{
		Get,
		Currency,
		ExistenceRequirement
	}
};
use frame_system::ensure_signed;
use sp_runtime::{
	DispatchResult,
	SaturatedConversion,
	traits::{
		AtLeast32BitUnsigned,
		Member,
	}
};
use codec::{Encode, Decode};


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod default_weight;

/// Chips info
#[derive(Encode, Decode, Default)]
pub struct ChipsDetail<Balance, Reserve> {
	/// Record the amount of available Balance
	pub balance: Balance,
	/// Record the amount of pledge deposits
	pub reserve: Reserve
}
pub trait WeightInfo {
	fn buy_chips() -> Weight;
	fn redemption() -> Weight;
}
pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Balances: Currency<Self::AccountId>;
	type ChipBalance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
	/// The address where funds are temporarily deposited
	type MasterAddress: Get<Self::AccountId>;
	type WeightInfo: WeightInfo;
}

pub type BalanceOf<T> = <<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;
decl_storage! {
	trait Store for Module<T: Config> as Chips {
		/// Record the chips information of each user
		pub ChipsMap get(fn chips_map): map hasher(blake2_128_concat)  T::AccountId => Option<ChipsDetail<T::ChipBalance, T::ChipBalance>>;
	}
}


decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, ChipBalance = <T as Config>::ChipBalance {
		/// Buy chips event
		BuyChips(AccountId, ChipBalance),
		/// Redemption amount with chips event
		Redemption(AccountId, ChipBalance),
		/// Pledge chips
		Reserve(AccountId, ChipBalance),
		/// Cancel pledge chips
		Unreserve(AccountId, ChipBalance),
		/// Transfer the chips in the pledge to others
		RepatriateReserved(AccountId, AccountId, ChipBalance),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		StorageOverflow,
		MoneyNotEnough,
		ChipsIsNotEnough,
		NeverBoughtChips
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		/// You can use your SGP to buy chips
		#[weight = T::WeightInfo::buy_chips()]
		pub fn buy_chips(origin, amount: T::ChipBalance) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;
			let master_address = T::MasterAddress::get();	// Receiving account
			let chips_map =  Self::chips_map(&_who);	// Get user chip information

			// The receiving account is forbidden to purchase and redeem to avoid errors
			ensure!(_who != master_address, Error::<T>::ChipsIsNotEnough);

			// payment
			// [Exchange] chips exchange for SGP
			let origin_amount = Self::exchange_chip_to_token(amount);
			T::Balances::transfer(&_who, &master_address, origin_amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough)?;


			
			// Redeem chips for the first time
			if chips_map.is_none() {	
				let c = ChipsDetail{
					balance: amount,
					reserve: 0_u128.saturated_into(),
				};
				<ChipsMap<T>>::insert(&_who, c);
			}else{
				// Return the Some value contained in it
				let c = chips_map.unwrap();
				let balance = c.balance + amount;
				let reserve = c.reserve;

				let new_chips = ChipsDetail{
					balance: balance,
					reserve: reserve,
				};
				<ChipsMap<T>>::mutate(&_who, |chips_detail| *chips_detail = Some(new_chips));
			}

			// Send event notification
			Self::deposit_event(RawEvent::BuyChips(_who, amount));
			Ok(())
		}

		/// You can use your chips to redemption SGP
		#[weight = T::WeightInfo::redemption()]
		pub fn redemption(origin, amount: T::ChipBalance) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;
			let master_address = T::MasterAddress::get();	// Receiving account
			// Get the balance of the chips, if you have not bought the chips, return an error
			let mut chips_map = Self::chips_map(&_who).ok_or( Error::<T>::NeverBoughtChips)?;

			// Need to have enough chips
			ensure!(chips_map.balance >= amount, Error::<T>::ChipsIsNotEnough);


			// Update chips
			chips_map.balance = chips_map.balance - amount;	
			<ChipsMap<T>>::mutate(&_who, |chips_detail| *chips_detail = Some(chips_map));

			// Ransom refund
			// 【Exchange】Use chips to exchange SGP
			let origin_amount = Self::exchange_chip_to_token(amount);
			T::Balances::transfer(&master_address, &_who, origin_amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough )?;
			// Send event notification
			Self::deposit_event(RawEvent::Redemption(_who, amount));
			Ok(())
		}
		
	}
}


impl<T: Config> Module<T> {
	/// chips to SGP
	fn exchange_chip_to_token(balance: T::ChipBalance) -> BalanceOf<T> {
		let u128_b = SaturatedConversion::saturated_into::<u128>(balance);
		let chip_balance:BalanceOf<T> = u128_b.saturated_into();

		// exchange rate
		let exchange = 1_u128.saturated_into();
		chip_balance * exchange
	}
}
pub trait ChipsTrait{
	type ChipBalance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
}
pub trait ChipsTransfer<AccountId> : ChipsTrait {
	fn reserve(account: &AccountId, balance: Self::ChipBalance) -> DispatchResult;
	fn unreserve(account: &AccountId, balance: Self::ChipBalance) -> DispatchResult;
	fn repatriate_reserved(from: &AccountId, to: &AccountId, balance: Self::ChipBalance) -> DispatchResult;
}

impl<T: Config> ChipsTrait for Module<T> {
	type ChipBalance = T::ChipBalance;
}

impl<T: Config> ChipsTransfer<T::AccountId> for Module<T> {
	/// 【chips action】Pledge chips
	fn reserve(_who: &T::AccountId, amount: Self::ChipBalance) -> dispatch::DispatchResult {
		// Get the balance of the chip, if it does not exist, return an error
		let mut chips_map = Self::chips_map(&_who).ok_or( Error::<T>::NeverBoughtChips)?;

		// Need to have enough chips
		ensure!(chips_map.balance >= amount, Error::<T>::ChipsIsNotEnough);
		
		// update chips
		chips_map.balance = chips_map.balance - amount;
		chips_map.reserve = chips_map.reserve + amount;
		<ChipsMap<T>>::mutate(&_who, |chips_detail| *chips_detail = Some(chips_map));

		// Send event notification
		RawEvent::Reserve(_who, amount);
		Ok(())
	}

	/// 【chips action】cancel pledge chips
	fn unreserve(_who: &T::AccountId, amount: Self::ChipBalance) -> dispatch::DispatchResult {
		// Get the balance of the chip, if it does not exist, return an error
		let mut chips_map = Self::chips_map(&_who).ok_or( Error::<T>::NeverBoughtChips)?;

		// Need to have enough pledge chips
		ensure!(chips_map.reserve >= amount, Error::<T>::ChipsIsNotEnough);
		
		// update chips
		chips_map.balance += amount;
		chips_map.reserve -= amount;
		<ChipsMap<T>>::mutate(&_who, |chips_detail| *chips_detail = Some(chips_map));

		// Send event notification
		RawEvent::Unreserve(_who, amount);
		Ok(())
	}

	/// 【chips action】Transfer the chips in the pledge to others
	fn repatriate_reserved(from: &T::AccountId, to: &T::AccountId, amount: Self::ChipBalance) -> dispatch::DispatchResult {
		//  Get the balance of the chip, if it does not exist, return an error
		let mut chips_from =  Self::chips_map(&from).ok_or( Error::<T>::NeverBoughtChips)?;	
		let mut chips_to =  Self::chips_map(&to).ok_or( Error::<T>::NeverBoughtChips)?;	

		// Need to have enough pledge chips
		ensure!(chips_from.reserve >= amount, Error::<T>::ChipsIsNotEnough);
		
		// from update chip
		chips_from.reserve -= amount;
		<ChipsMap<T>>::mutate(&from, |chips_detail| *chips_detail = Some(chips_from));

		// to update chip
		chips_to.balance += amount;
		<ChipsMap<T>>::mutate(&to, |chips_detail| *chips_detail = Some(chips_to));

		// Send event notification
		RawEvent::RepatriateReserved(from, to, amount);
		Ok(())
	}
}