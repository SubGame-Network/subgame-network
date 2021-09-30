//! SubGame Swap Dapp

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, Parameter,
	traits::{Currency, ReservableCurrency, ExistenceRequirement},
	weights::{Weight, Pays, DispatchClass},
};
use sp_runtime::{
	ModuleId,
	traits::{
		Member, Zero, AtLeast32Bit, MaybeSerializeDeserialize, CheckedAdd,
		AccountIdConversion, SaturatedConversion, 
	}
};
use frame_system::ensure_signed;
use pallet_subgame_assets::{self as SubGameAssets};

#[allow(unused_imports)]
use num_traits::float::FloatCore;
#[allow(unused_imports)]
use micromath::F32Ext;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod default_weight;

#[macro_use]
extern crate alloc;

pub trait WeightInfo {
	fn create_pool() -> Weight;
	fn add_liquidity() -> Weight;
	fn remove_liquidity() -> Weight;
	fn swap() -> Weight;
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SwapPoolDetails<SwapId, AccountId, AssetId> {
	// The pool id.
	swap_id: SwapId,
	// The swap account.
	account: AccountId,
	// asset x
	asset_x: AssetId,
	// asset y
	asset_y: AssetId,
	// asset LP Token
	asset_lp: AssetId,
	// k
	swap_k: u128
}

/// There is a 0.3% fee for swapping tokens.
const FEE: f64 = 0.003;

/// The swap's module id, used for deriving sovereign account IDs.
const MODULE_ID: ModuleId = ModuleId(*b"mtg/swap");

/// LP token decimals
pub const SGB_DECIMALS: u64 = 10_000_000_000;
/// LP token decimals
pub const LP_DECIMALS: u64 = 1_000_000;

pub trait Config: frame_system::Config + SubGameAssets::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type WeightInfo: WeightInfo;
	type SwapId: Parameter + Member + AtLeast32Bit + Default + Copy + MaybeSerializeDeserialize;
	type Currency: ReservableCurrency<Self::AccountId>;
}

decl_storage! {
	trait Store for Module<T: Config> as Swap {
		pub SwapPair get(fn swap_pair): map hasher(blake2_128_concat) (T::AssetId, T::AssetId) => T::SwapId;
		pub SwapPoolCount get(fn swap_pool_count): T::SwapId;
		pub SwapPool get(fn swap_pool): map hasher(blake2_128_concat) T::SwapId => SwapPoolDetails<T::SwapId, T::AccountId, T::AssetId>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		SwapSender = <T as frame_system::Config>::AccountId,
		SwapPoolOwner = <T as frame_system::Config>::AccountId,
		SwapAssetX = <T as SubGameAssets::Config>::AssetId,
		SwapAssetY = <T as SubGameAssets::Config>::AssetId,
		SwapId = <T as Config>::SwapId,
		SwapAmountX = <T as SubGameAssets::Config>::SGAssetBalance,
		SwapAmountY = <T as SubGameAssets::Config>::SGAssetBalance,
		SwapAmountLP = <T as SubGameAssets::Config>::SGAssetBalance,
	{
		CreatePool(SwapSender, SwapId, SwapAssetX, SwapAmountX, SwapAssetY, SwapAmountY, SwapPoolOwner),
		LiquidityAdded(SwapId, SwapSender, SwapAmountX, SwapAmountY),
		LiquidityRemoved(SwapId, SwapSender, SwapAmountLP, SwapAmountX, SwapAmountY),
		Swap(SwapId, SwapSender, SwapAssetX, SwapAmountX, SwapAssetY, SwapAmountY),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Asset not found.
		AssetNotFound,
		/// Input duplicate asset_id.
		DuplicateAssetId,
		/// A Swap already exists.
		SwapAlreadyExists,
		/// Checked add swap pool count error.
		PoolCountError,
		/// No Swap exists at this Id.
		NoSwapExists,
		/// Zero balance supplied.
		ZeroBalance,
		/// liquidity k not match error.
		LiquidityKError,
		/// Not enough liquidity to transfer.
		NotEnoughLiquidity,
		/// Not enough LP token.
		NotEnoughLPToken,
		/// Not enough balance.
		NotEnoughBalance,
		/// Deadline hit.
		Deadline,
		/// Slipage hit.
		Slipage,
		/// expected swap output amount can not be zero.
		ZeroExpectedAmount,
		/// Too many LP token.
		TooManyLPToken
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = <T as Config>::WeightInfo::create_pool()]
		pub fn create_pool(
			origin, 
			asset_x: T::AssetId,
			x: T::SGAssetBalance,
			asset_y: T::AssetId,
			y: T::SGAssetBalance
		) -> dispatch::DispatchResult
		{
			let sender = ensure_signed(origin)?;
			ensure!(asset_x != asset_y, Error::<T>::DuplicateAssetId);
			ensure!(!SwapPair::<T>::contains_key((asset_x, asset_y)), Error::<T>::SwapAlreadyExists);
			ensure!(!SwapPair::<T>::contains_key((asset_y, asset_x)), Error::<T>::SwapAlreadyExists);
			ensure!(x.saturated_into::<u64>() > 0, Error::<T>::ZeroBalance);
			ensure!(y.saturated_into::<u64>() > 0, Error::<T>::ZeroBalance);
			
			let origin_coin: T::AssetId = 0u32.into();

			if asset_x == origin_coin {
				ensure!(<T as Config>::Currency::free_balance(&sender).saturated_into::<u64>() >= x.saturated_into::<u64>(), Error::<T>::NotEnoughBalance);
			} else {
				ensure!(SubGameAssets::Module::<T>::balance(asset_x, sender.clone()) >= x, Error::<T>::NotEnoughBalance);
			}

			if asset_y == origin_coin {
				ensure!(<T as Config>::Currency::free_balance(&sender).saturated_into::<u64>() >= y.saturated_into::<u64>(), Error::<T>::NotEnoughBalance);
			} else {
				ensure!(SubGameAssets::Module::<T>::balance(asset_y, sender.clone()) >= y, Error::<T>::NotEnoughBalance);
			}

			let new_pool_id = SwapPoolCount::<T>::get().checked_add(&1u32.into()).ok_or(Error::<T>::PoolCountError)?;
			let pool_account: T::AccountId = MODULE_ID.into_sub_account(new_pool_id);
			
			let mut _no_decimal_x: f64 = x.saturated_into::<u64>() as f64 / SGB_DECIMALS as f64;
			let metadata_x = SubGameAssets::Metadata::<T>::get(asset_x);
			if asset_x != origin_coin {
				ensure!(metadata_x.decimals > 0, Error::<T>::AssetNotFound);
				
				let base_decimal: i64 = 10;
				_no_decimal_x = x.saturated_into::<u64>() as f64 / base_decimal.pow(metadata_x.decimals.saturated_into::<u32>()) as f64;
			}

			let mut _no_decimal_y: f64 = y.saturated_into::<u64>() as f64 / SGB_DECIMALS as f64;
			let metadata_y = SubGameAssets::Metadata::<T>::get(asset_y);
			if asset_y != origin_coin {
				ensure!(metadata_y.decimals > 0, Error::<T>::AssetNotFound);

				let base_decimal: i64 = 10;
				_no_decimal_y = y.saturated_into::<u64>() as f64 / base_decimal.pow(metadata_y.decimals.saturated_into::<u32>()) as f64;
			}

			// SwapPoolCount
			SwapPoolCount::<T>::put(new_pool_id);

			// SwapPair
			SwapPair::<T>::insert((asset_x, asset_y), new_pool_id);
			SwapPair::<T>::insert((asset_y, asset_x), new_pool_id);
			
			// LP token balance
			let lp_balance: u64 = ((_no_decimal_x as f32 * _no_decimal_y as f32).sqrt() as f64 * LP_DECIMALS as f64).floor() as u64;
			
			// LP asset id
			let lp_asset_id: T::AssetId = frame_system::Module::<T>::block_number().saturated_into::<u32>().into();

			// SwapPool
			let pool_details = SwapPoolDetails{
				swap_id: new_pool_id,
				account: pool_account.clone(),
				asset_x: asset_x,
				asset_y: asset_y,
				asset_lp: lp_asset_id,
				swap_k: (x.saturated_into::<u128>() * y.saturated_into::<u128>())
			};
			SwapPool::<T>::insert(new_pool_id, pool_details);
			
			// Create LP Token
			let max_zombies: u32 = 999999999;
        	let min_balance: u32 = 1;
			SubGameAssets::Module::<T>::_force_create(lp_asset_id, pool_account.clone(), max_zombies, min_balance.into())?;
			let mut symbol_x = "SGB";
			if asset_x != origin_coin {
				symbol_x = core::str::from_utf8(&metadata_x.symbol).unwrap();
			}
			let mut symbol_y = "SGB";
			if asset_y != origin_coin {
				symbol_y = core::str::from_utf8(&metadata_y.symbol).unwrap();
			}
			let lp_name = format!("{}-{} LP", symbol_x, symbol_y);
			SubGameAssets::Module::<T>::_force_set_metadata(pool_account.clone(), lp_asset_id, lp_name.as_bytes().to_vec(), lp_name.as_bytes().to_vec(), 6)?;
			SubGameAssets::Module::<T>::_mint(pool_account.clone(), lp_asset_id, sender.clone(), lp_balance.saturated_into())?;

			// transfer x
			if asset_x == origin_coin {
				let _balance: u64 = x.saturated_into::<u64>();
				let _pool_account = pool_account.clone();
				<T as Config>::Currency::transfer(&sender, &_pool_account, _balance.saturated_into(), ExistenceRequirement::KeepAlive)?;
			} else {
				SubGameAssets::Module::<T>::_transfer(sender.clone(), asset_x, pool_account.clone(), x)?;
			}

			// transfer y
			if asset_y == origin_coin {
				let _balance: u64 = y.saturated_into::<u64>();
				let _pool_account = pool_account.clone();
				<T as Config>::Currency::transfer(&sender, &_pool_account, _balance.saturated_into(), ExistenceRequirement::KeepAlive)?;
			} else {
				SubGameAssets::Module::<T>::_transfer(sender.clone(), asset_y, pool_account.clone(), y)?;
			}

			Self::deposit_event(RawEvent::CreatePool(sender, new_pool_id, asset_x, x, asset_y, y, pool_account));
			Ok(())
		}

		#[weight = <T as Config>::WeightInfo::add_liquidity()]
		pub fn add_liquidity(
			origin,
		    swap_id: T::SwapId,
			dx: T::SGAssetBalance,
			dy: T::SGAssetBalance
		) -> dispatch::DispatchResult
		{
		    let sender = ensure_signed(origin.clone())?;

			let swap_pool = SwapPool::<T>::get(swap_id);
			ensure!(swap_pool.swap_id == swap_id, Error::<T>::NoSwapExists);
			ensure!(dx.saturated_into::<u64>() > 0 && dy.saturated_into::<u64>() > 0, Error::<T>::ZeroBalance);
			
			let origin_coin: T::AssetId = 0u32.into();

			if swap_pool.asset_x == origin_coin {
				ensure!(<T as Config>::Currency::free_balance(&sender).saturated_into::<u64>() >= dx.saturated_into::<u64>(), Error::<T>::NotEnoughBalance);
			} else {
				ensure!(SubGameAssets::Module::<T>::balance(swap_pool.asset_x, sender.clone()) >= dx, Error::<T>::NotEnoughBalance);
			}

			if swap_pool.asset_y == origin_coin {
				ensure!(<T as Config>::Currency::free_balance(&sender).saturated_into::<u64>() >= dy.saturated_into::<u64>(), Error::<T>::NotEnoughBalance);
			} else {
				ensure!(SubGameAssets::Module::<T>::balance(swap_pool.asset_y, sender.clone()) >= dy, Error::<T>::NotEnoughBalance);
			}

			let mut x: u64 = SubGameAssets::Module::<T>::balance(swap_pool.asset_x, swap_pool.account.clone()).saturated_into::<u64>();
			if swap_pool.asset_x == origin_coin {
				x = <T as Config>::Currency::free_balance(&swap_pool.account.clone()).saturated_into::<u64>();
			}

			let mut y: u64 = SubGameAssets::Module::<T>::balance(swap_pool.asset_y, swap_pool.account.clone()).saturated_into::<u64>();
			if swap_pool.asset_y == origin_coin {
				y = <T as Config>::Currency::free_balance(&swap_pool.account.clone()).saturated_into::<u64>();
			}
			
			let total_liquidity = x + y;
			if total_liquidity > Zero::zero() {
				// LP total supply
				let lp_total_supply = SubGameAssets::Module::<T>::total_supply(swap_pool.asset_lp);

				// mint LP token y
				let new_lp_balance_x: u64 = (dx.saturated_into::<u64>() as f64 / x as f64 * lp_total_supply.saturated_into::<u64>() as f64).floor() as u64;
				let want_dy: u64 = (new_lp_balance_x as f64 / lp_total_supply.saturated_into::<u64>() as f64 * y as f64).floor() as u64;

				// mint LP token x
				let new_lp_balance_y: u64 = (dy.saturated_into::<u64>() as f64 / y as f64 * lp_total_supply.saturated_into::<u64>() as f64).floor() as u64;
				let want_dx: u64 = (new_lp_balance_y as f64 / lp_total_supply.saturated_into::<u64>() as f64 * x as f64).floor() as u64;

				// Check K
				ensure!(want_dy == dy.saturated_into::<u64>() || want_dx == dx.saturated_into::<u64>(), Error::<T>::LiquidityKError);
				
				let mut new_lp_balance = new_lp_balance_x;
				if want_dx == dx.saturated_into::<u64>() {
					new_lp_balance = new_lp_balance_y;
				}

				// transfer x
				if swap_pool.asset_x == origin_coin {
					let _balance: u64 = dx.saturated_into::<u64>();
					<T as Config>::Currency::transfer(&sender, &swap_pool.account.clone(), _balance.saturated_into(), ExistenceRequirement::KeepAlive)?;
				} else {
					SubGameAssets::Module::<T>::_transfer(sender.clone(), swap_pool.asset_x, swap_pool.account.clone(), dx)?;
				}

				// transfer y
				if swap_pool.asset_y == origin_coin {
					let _balance: u64 = dy.saturated_into::<u64>();
					<T as Config>::Currency::transfer(&sender, &swap_pool.account.clone(), _balance.saturated_into(), ExistenceRequirement::KeepAlive)?;
				} else {
					SubGameAssets::Module::<T>::_transfer(sender.clone(), swap_pool.asset_y, swap_pool.account.clone(), dy)?;
				}

				// mint LP token
				SubGameAssets::Module::<T>::_mint(swap_pool.account.clone(), swap_pool.asset_lp, sender.clone(), new_lp_balance.saturated_into())?;
			} else {
				// transfer x
				if swap_pool.asset_x == origin_coin {
					let _balance: u64 = dx.saturated_into::<u64>();
					<T as Config>::Currency::transfer(&sender, &swap_pool.account.clone(), _balance.saturated_into(), ExistenceRequirement::KeepAlive)?;
				} else {
					SubGameAssets::Module::<T>::_transfer(sender.clone(), swap_pool.asset_x, swap_pool.account.clone(), dx)?;
				}

				// transfer y
				if swap_pool.asset_y == origin_coin {
					let _balance: u64 = dy.saturated_into::<u64>();
					<T as Config>::Currency::transfer(&sender, &swap_pool.account.clone(), _balance.saturated_into(), ExistenceRequirement::KeepAlive)?;
				} else {
					SubGameAssets::Module::<T>::_transfer(sender.clone(), swap_pool.asset_y, swap_pool.account.clone(), dy)?;
				}

				// mint LP token
				let new_lp_balance = (dx + dy) / 2u32.into();
				SubGameAssets::Module::<T>::_mint(swap_pool.account.clone(), swap_pool.asset_lp, sender.clone(), new_lp_balance)?;
			}

			Self::deposit_event(RawEvent::LiquidityAdded(swap_id, sender.clone(), dx, dy));
			Ok(())
		}

		#[weight = (<T as Config>::WeightInfo::remove_liquidity(), DispatchClass::Normal, Pays::No)]
		pub fn remove_liquidity(
			origin,
		    swap_id: T::SwapId,
			lp_amount: T::SGAssetBalance
		) -> dispatch::DispatchResult
		{
		    let sender = ensure_signed(origin.clone())?;
			ensure!(lp_amount.saturated_into::<u64>() > 0, Error::<T>::ZeroBalance);

			let swap_pool = SwapPool::<T>::get(swap_id);
			ensure!(swap_pool.swap_id == swap_id, Error::<T>::NoSwapExists);

			let sender_lp_balance = SubGameAssets::Module::<T>::balance(swap_pool.asset_lp, sender.clone());
			ensure!(sender_lp_balance >= lp_amount, Error::<T>::NotEnoughLPToken);
			
			let origin_coin: T::AssetId = 0u32.into();

			let mut x: u64 = SubGameAssets::Module::<T>::balance(swap_pool.asset_x, swap_pool.account.clone()).saturated_into::<u64>();
			if swap_pool.asset_x == origin_coin {
				x = <T as Config>::Currency::free_balance(&swap_pool.account.clone()).saturated_into::<u64>();
			}
			ensure!(!x.is_zero(), Error::<T>::NotEnoughLiquidity);

			let mut y: u64 = SubGameAssets::Module::<T>::balance(swap_pool.asset_y, swap_pool.account.clone()).saturated_into::<u64>();
			if swap_pool.asset_y == origin_coin {
				y = <T as Config>::Currency::free_balance(&swap_pool.account.clone()).saturated_into::<u64>();
			}
			ensure!(!y.is_zero(), Error::<T>::NotEnoughLiquidity);

			// LP total supply
			let lp_total_supply = SubGameAssets::Module::<T>::total_supply(swap_pool.asset_lp).saturated_into::<u64>();

			// Check LP token limit
			let limit_lp: f64 = lp_amount.saturated_into::<u64>() as f64 / lp_total_supply as f64;
			ensure!(limit_lp < 1.0f64, Error::<T>::TooManyLPToken);

			let dx: T::SGAssetBalance = ((lp_amount.saturated_into::<u64>() as f64 / lp_total_supply as f64 * x as f64).floor() as u64).saturated_into();
			let dy: T::SGAssetBalance = ((lp_amount.saturated_into::<u64>() as f64 / lp_total_supply as f64 * y as f64).floor() as u64).saturated_into();
			ensure!(!dx.is_zero() && !dy.is_zero(), Error::<T>::ZeroBalance);
			
			// transfer x
			if swap_pool.asset_x == origin_coin {
				let _balance: u64 = dx.saturated_into::<u64>();
				<T as Config>::Currency::transfer(&swap_pool.account.clone(), &sender, _balance.saturated_into(), ExistenceRequirement::AllowDeath)?;
			} else {
				SubGameAssets::Module::<T>::_transfer(swap_pool.account.clone(), swap_pool.asset_x, sender.clone(), dx)?;
			}

			// transfer y
			if swap_pool.asset_y == origin_coin {
				let _balance: u64 = dy.saturated_into::<u64>();
				<T as Config>::Currency::transfer(&swap_pool.account.clone(), &sender, _balance.saturated_into(), ExistenceRequirement::AllowDeath)?;
			} else {
				SubGameAssets::Module::<T>::_transfer(swap_pool.account.clone(), swap_pool.asset_y, sender.clone(), dy)?;
			}

			// burn LP token
			SubGameAssets::Module::<T>::_burn(swap_pool.account.clone(), swap_pool.asset_lp, sender.clone(), lp_amount)?;

			Self::deposit_event(RawEvent::LiquidityRemoved(swap_id, sender.clone(), lp_amount, dx, dy));
			Ok(())
		}

		#[weight = (<T as Config>::WeightInfo::swap(), DispatchClass::Normal, Pays::No)]
		pub fn swap(
			origin,
		    swap_id: T::SwapId,
			input_asset: T::AssetId,
			input_amount: T::SGAssetBalance,
			output_asset: T::AssetId,
			expected_output_amount: T::SGAssetBalance,
			slipage: u64,
			deadline: T::BlockNumber,
		) -> dispatch::DispatchResult
		{
			if deadline.saturated_into::<u32>() > 0u32 {
				let now = frame_system::Module::<T>::block_number();
				ensure!(deadline >= now, Error::<T>::Deadline);
			}

			let sender = ensure_signed(origin.clone())?;

			let swap_pool = SwapPool::<T>::get(swap_id);
			ensure!(swap_pool.swap_id == swap_id, Error::<T>::NoSwapExists);
			ensure!(input_asset != output_asset, Error::<T>::DuplicateAssetId);
			ensure!(swap_pool.asset_x == input_asset || swap_pool.asset_y == input_asset, Error::<T>::AssetNotFound);
			ensure!(swap_pool.asset_x == output_asset || swap_pool.asset_y == output_asset, Error::<T>::AssetNotFound);
			ensure!(expected_output_amount.saturated_into::<u64>() > 0u64, Error::<T>::ZeroExpectedAmount);
			ensure!(input_amount.saturated_into::<u64>() > 0u64, Error::<T>::ZeroBalance);
			
			let origin_coin: T::AssetId = 0u32.into();

			let mut input_balance: u64 = SubGameAssets::Module::<T>::balance(input_asset, swap_pool.account.clone()).saturated_into::<u64>();
			if input_asset == origin_coin {
				input_balance = <T as Config>::Currency::free_balance(&swap_pool.account.clone()).saturated_into::<u64>();
			}

			let mut output_balance: u64 = SubGameAssets::Module::<T>::balance(output_asset, swap_pool.account.clone()).saturated_into::<u64>();
			if output_asset == origin_coin {
				output_balance = <T as Config>::Currency::free_balance(&swap_pool.account.clone()).saturated_into::<u64>();
			}

			// $r = 1 - 0.003
			// $a = $dx / $x
			// $dy = ($a * $r) / (1 + ($a * $r)) * $y
			let x: f64 = input_balance as f64;
			let y: f64 = output_balance as f64;
			let dx: f64 = input_amount.saturated_into::<u64>() as f64;
			let r: f64 = 1.0 - FEE;
			let a: f64 = dx / x;
			let dy: f64 = (a * r) / (1.0 + (a * r)) * y;
			let output_amount: u64 = (dy.floor() as u64).saturated_into();
			ensure!(output_amount > 0u64.into(), Error::<T>::NotEnoughLiquidity);

			// slipage
			let _expected_output_amount: u64 = expected_output_amount.saturated_into::<u64>();
			if _expected_output_amount > 0u64 && slipage > 0u64 {
				let got_slipage: f64  = (_expected_output_amount as i64 - output_amount as i64).abs() as f64 / _expected_output_amount as f64 * 100f64;
				ensure!(slipage as f64 >= got_slipage  , Error::<T>::Slipage);
			}

			// transfer input
			if input_asset == origin_coin {
				let _balance: u64 = input_amount.saturated_into::<u64>();
				<T as Config>::Currency::transfer(&sender, &swap_pool.account.clone(), _balance.saturated_into(), ExistenceRequirement::AllowDeath)?;
			} else {
				SubGameAssets::Module::<T>::_transfer(sender.clone(), input_asset, swap_pool.account.clone(), input_amount)?;
			}

			// transfer output
			if output_asset == origin_coin {
				<T as Config>::Currency::transfer(&swap_pool.account.clone(), &sender, output_amount.saturated_into(), ExistenceRequirement::AllowDeath)?;
			} else {
				SubGameAssets::Module::<T>::_transfer(swap_pool.account.clone(), output_asset, sender.clone(), output_amount.saturated_into())?;
			}

			Self::deposit_event(RawEvent::Swap(swap_id, sender.clone(), input_asset, input_amount, output_asset, output_amount.saturated_into()));
			Ok(())
		}
	}
}

impl<T: Config> Module<T> {
	
}