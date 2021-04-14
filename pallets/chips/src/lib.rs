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

#[derive(Encode, Decode, Default)]
pub struct ChipsDetail<Balance, Reserve> {
	pub balance: Balance,
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
	type MasterAddress: Get<Self::AccountId>;
	type WeightInfo: WeightInfo;
}

pub type BalanceOf<T> = <<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;
decl_storage! {
	trait Store for Module<T: Config> as Chips {
		pub ChipsMap get(fn chips_map): map hasher(blake2_128_concat)  T::AccountId => Option<ChipsDetail<T::ChipBalance, T::ChipBalance>>;
	}
}


decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, ChipBalance = <T as Config>::ChipBalance {
		BuyChips(AccountId, ChipBalance),
		Redemption(AccountId, ChipBalance),
		Reserve(AccountId, ChipBalance),
		Unreserve(AccountId, ChipBalance),
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
		// 只要有用到Error就要這行
		type Error = Error<T>;

		// 只要有用到Event就要這行
		fn deposit_event() = default;

		// 購買籌碼
		#[weight = T::WeightInfo::buy_chips()]
		pub fn buy_chips(origin, amount: T::ChipBalance) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;
			let master_address = T::MasterAddress::get();	// 收款帳號
			let chips_map =  Self::chips_map(&_who);	// 取得籌碼資料

			// 收款帳號不能購買、贖回，以免發生錯誤
			ensure!(_who != master_address, Error::<T>::ChipsIsNotEnough);

			// 收款
			// 【換匯】籌碼換原生幣
			let origin_amount = Self::exchange_chip_to_token(amount);
			T::Balances::transfer(&_who, &master_address, origin_amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough)?;


			
			// 首次兌換籌碼
			if chips_map.is_none() {	
				let c = ChipsDetail{
					balance: amount,
					reserve: 0_u128.saturated_into(),
				};
				<ChipsMap<T>>::insert(&_who, c);
			}else{
				// 返回包含在其中的Some值
				let c = chips_map.unwrap();
				let balance = c.balance + amount;
				let reserve = c.reserve;

				let new_chips = ChipsDetail{
					balance: balance,
					reserve: reserve,
				};
				<ChipsMap<T>>::mutate(&_who, |chips_detail| *chips_detail = Some(new_chips));
			}

			// 發送事件通知
			Self::deposit_event(RawEvent::BuyChips(_who, amount));
			Ok(())
		}

		// 贖回
		#[weight = T::WeightInfo::redemption()]
		pub fn redemption(origin, amount: T::ChipBalance) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;
			let master_address = T::MasterAddress::get();	// 收款帳號
			// 取得籌碼餘額，若沒買過籌碼返回錯誤
			let mut chips_map = Self::chips_map(&_who).ok_or( Error::<T>::NeverBoughtChips)?;

			// 擁有足夠的籌碼
			ensure!(chips_map.balance >= amount, Error::<T>::ChipsIsNotEnough);


			// 更新籌碼
			chips_map.balance = chips_map.balance - amount;	
			<ChipsMap<T>>::mutate(&_who, |chips_detail| *chips_detail = Some(chips_map));

			// 退回贖金
			// 【換匯】籌碼換原生幣
			let origin_amount = Self::exchange_chip_to_token(amount);
			T::Balances::transfer(&master_address, &_who, origin_amount, ExistenceRequirement::KeepAlive).map_err(|_| Error::<T>::MoneyNotEnough )?;
			// 發送事件通知
			Self::deposit_event(RawEvent::Redemption(_who, amount));
			Ok(())
		}
		
	}
}


impl<T: Config> Module<T> {
	// 籌碼換原生幣
	fn exchange_chip_to_token(balance: T::ChipBalance) -> BalanceOf<T> {
		let u128_b = SaturatedConversion::saturated_into::<u128>(balance);
		let chip_balance:BalanceOf<T> = u128_b.saturated_into();

		// 匯率
		let exchange = 1_u128.saturated_into();
		chip_balance * exchange
	}
}
pub trait ChipsTrait{
	// 定義籌碼餘額type
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
	
	// 【籌碼操作】質押
	fn reserve(_who: &T::AccountId, amount: Self::ChipBalance) -> dispatch::DispatchResult {
		// 取得籌碼餘額，若不存在則返回錯誤
		let mut chips_map = Self::chips_map(&_who).ok_or( Error::<T>::NeverBoughtChips)?;

		// 擁有足夠的籌碼
		ensure!(chips_map.balance >= amount, Error::<T>::ChipsIsNotEnough);
		
		// 更新籌碼
		chips_map.balance = chips_map.balance - amount;
		chips_map.reserve = chips_map.reserve + amount;
		<ChipsMap<T>>::mutate(&_who, |chips_detail| *chips_detail = Some(chips_map));

		// 發送事件
		RawEvent::Reserve(_who, amount);
		Ok(())
	}

	// 【籌碼操作】質押取回
	fn unreserve(_who: &T::AccountId, amount: Self::ChipBalance) -> dispatch::DispatchResult {
		// 取得籌碼餘額，若不存在則返回錯誤
		let mut chips_map = Self::chips_map(&_who).ok_or( Error::<T>::NeverBoughtChips)?;

		// 擁有足夠的質押籌碼
		ensure!(chips_map.reserve >= amount, Error::<T>::ChipsIsNotEnough);
		
		// 更新籌碼
		chips_map.balance += amount;
		chips_map.reserve -= amount;
		<ChipsMap<T>>::mutate(&_who, |chips_detail| *chips_detail = Some(chips_map));

		// 發送事件
		RawEvent::Unreserve(_who, amount);
		Ok(())
	}

	// 【籌碼操作】轉移
	fn repatriate_reserved(from: &T::AccountId, to: &T::AccountId, amount: Self::ChipBalance) -> dispatch::DispatchResult {
		// 取得籌碼餘額，若不存在則返回錯誤
		let mut chips_from =  Self::chips_map(&from).ok_or( Error::<T>::NeverBoughtChips)?;	
		let mut chips_to =  Self::chips_map(&to).ok_or( Error::<T>::NeverBoughtChips)?;	

		// 擁有足夠的質押籌碼
		ensure!(chips_from.reserve >= amount, Error::<T>::ChipsIsNotEnough);
		
		// from 更新籌碼
		chips_from.reserve -= amount;
		<ChipsMap<T>>::mutate(&from, |chips_detail| *chips_detail = Some(chips_from));

		// to 更新籌碼
		chips_to.balance += amount;
		<ChipsMap<T>>::mutate(&to, |chips_detail| *chips_detail = Some(chips_to));

		// 發送事件
		RawEvent::RepatriateReserved(from, to, amount);
		Ok(())
	}
}