#![cfg_attr(not(feature = "std"), no_std)]

pub mod default_weight;
use default_weight::WeightInfo;


use sp_std::{prelude::*};
use sp_runtime::{RuntimeDebug};
use codec::{Encode, Decode};
use frame_support::{decl_module, decl_event, decl_storage, decl_error, ensure,
	traits::{Get},
	dispatch::{DispatchResult},
};
use frame_system::ensure_signed;


use pallet_lease::Lease;

use pallet_nft::UniqueAssets;

use pallet_subgame_assets::{self as SubGameAssets};
use pallet_subgame_assets::{AssetsTrait, AssetsTransfer};

pub type NftId<T> = 
    <<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetId;

pub type PalletId<T> = 
    <<T as Config>::Lease as Lease<<T as frame_system::Config>::AccountId, NftId<T>>>::PalletId;


/// 設定優惠的儲值金額和積分方案
/// Set discounted stored value amount and point plan
#[derive(Clone, Encode, Decode, Copy, Eq, PartialEq, RuntimeDebug, Default)]
pub struct Plan<SGAssetBalance> {
	pub amount: SGAssetBalance,
	pub score: SGAssetBalance,
}

/// ability increases with level(the increased ability is a random number in the interval)
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct GRPlatform<Account, AssetId, Plan> {
	pub id: u128,
	pub admin: Account,
	pub pool_account: Account, // 要補錢進去
	pub asset_id: AssetId,
	pub plan: Vec<Plan>, // 允許使用的優惠方案(儲值時若無匹配，則1:1上分)
}

/// The module configuration trait.
pub trait Config: frame_system::Config + SubGameAssets::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	
    type UniqueAssets: UniqueAssets<Self::AccountId>;
    type Assets: AssetsTrait + AssetsTransfer<Self::AccountId, u32>;
    type Lease: Lease<Self::AccountId, NftId<Self>>;
    type PalletId: Get<PalletId<Self>>;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}



decl_storage! {
	trait Store for Module<T: Config> as GameRecharge {
		pub NextPlatformId get(fn next_platform_id): u128 = 1;

		pub Platforms get(fn platform_by_id): map hasher(blake2_128_concat) u128 => Option<GRPlatform<T::AccountId, T::AssetId, Plan<T::SGAssetBalance>>>;
		
	}
}

decl_event! {
	pub enum Event<T> where
		AccountId = <T as frame_system::Config>::AccountId,
		AssetId = <T as SubGameAssets::Config>::AssetId,
		SGAssetBalance = <T as SubGameAssets::Config>::SGAssetBalance,
	{
		NewPlatform(u128,AccountId,AccountId,AssetId,Vec<Plan<SGAssetBalance>>),
		UpdatePlatform(AccountId,u128,Vec<Plan<SGAssetBalance>>),
		Recharge(AccountId,u128,SGAssetBalance,SGAssetBalance,u128),
		Withdraw(AccountId,SGAssetBalance,u128),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		PermissionDenied,
		NotFoundAsset,
		NotFoundData,
		NotAdmin,
		UnknownPlatform,
		BalanceNotEnough,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = <T as Config>::WeightInfo::create_platform()]
		fn create_platform(origin,
			admin: T::AccountId,
			pool_account: T::AccountId,
			asset_id: T::AssetId,
			plan: Vec<Plan<T::SGAssetBalance>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);

			// check asset_id exist
			let asset = SubGameAssets::Asset::<T>::get(asset_id);
			ensure!(asset != None, Error::<T>::NotFoundAsset);
			
			let platform_id = Self::next_platform_id();

			Platforms::<T>::insert(platform_id, GRPlatform {
				id: platform_id,
				admin:admin.clone(),
				pool_account: pool_account.clone(),
				asset_id: asset_id.clone(),
				plan: plan.clone(),
			});

			NextPlatformId::mutate(|platform_id| *platform_id += 1);

			Self::deposit_event(RawEvent::NewPlatform(
				platform_id,
				admin,
				pool_account,
				asset_id,
				plan,
			));
			Ok(())
			
		}

		#[weight = <T as Config>::WeightInfo::update_platform()]
		fn update_platform(origin,
			id: u128,
			plan: Vec<Plan<T::SGAssetBalance>>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Platforms::<T>::try_mutate_exists(id, |platform| {
				let _platform = platform.as_mut().ok_or( Error::<T>::NotFoundData)?;
				ensure!(_platform.admin == sender, Error::<T>::NotAdmin);
	
				_platform.plan = plan.clone();
	
				Self::deposit_event(RawEvent::UpdatePlatform(
					sender,
					id,
					plan,
				));
				Ok(())
			})
		}
		#[weight = <T as Config>::WeightInfo::platform_change_admin()]
		fn platform_change_admin(origin,
			platform_id: u128,
			new_admin: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Platforms::<T>::try_mutate_exists(platform_id, |platform| {
				let _platform = platform.as_mut().ok_or( Error::<T>::NotFoundData)?;
				ensure!(_platform.admin == sender, Error::<T>::NotAdmin);
				_platform.admin = new_admin;
				Ok(())
			})
		}

		#[weight = <T as Config>::WeightInfo::recharge()]
		fn recharge(origin,
			platform_id: u128,
			match_id: u128,
			amount: T::SGAssetBalance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check platform exist
			let _platform = Platforms::<T>::get(platform_id).ok_or(Error::<T>::UnknownPlatform)?;

            let plan = _platform.plan
			.iter()
			.find(|&&probe| probe.amount == amount);

			let mut score = amount;
			if plan != None {
				score = plan.unwrap().score;
			}

			ensure!(SubGameAssets::Module::<T>::balance(_platform.asset_id, sender.clone()) >= amount, Error::<T>::BalanceNotEnough);

			SubGameAssets::Module::<T>::_transfer(sender.clone(), _platform.asset_id, _platform.pool_account.clone(), amount)?;


			Self::deposit_event(RawEvent::Recharge(
				sender,
				match_id,
				amount,
				score,
				platform_id,
			));
			Ok(())
		}
		
		#[weight = <T as Config>::WeightInfo::withdraw()]
		fn withdraw(origin,
			platform_id: u128,
			target: T::AccountId,
			amount: T::SGAssetBalance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			// check platform exist
			let _platform = Platforms::<T>::get(platform_id).ok_or(Error::<T>::UnknownPlatform)?;
			ensure!(_platform.admin == sender, Error::<T>::NotAdmin);

			ensure!(SubGameAssets::Module::<T>::balance(_platform.asset_id, _platform.pool_account.clone()) >= amount, Error::<T>::BalanceNotEnough);

			SubGameAssets::Module::<T>::_transfer(_platform.pool_account, _platform.asset_id, target.clone(), amount)?;
			
			Self::deposit_event(RawEvent::Withdraw(
				target,
				amount,
				platform_id
			));
			Ok(())
		}
		
	}
}