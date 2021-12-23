#![cfg_attr(not(feature = "std"), no_std)]

pub mod default_weight;
use default_weight::WeightInfo;


use sp_std::{prelude::*};

use frame_support::{decl_module, decl_event, decl_storage, decl_error, ensure,
	traits::{Currency, ExistenceRequirement, Get},
	dispatch::{DispatchResult},
};
use frame_system::ensure_signed;

pub mod nft_exchange;
pub use crate::nft_exchange::*;


use pallet_lease::Lease;

use pallet_nft::UniqueAssets;

pub type NftId<T> = <<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetId;

pub type BalanceOf<T> =
    <<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type PalletId<T> = 
    <<T as Config>::Lease as Lease<<T as frame_system::Config>::AccountId, NftId<T>>>::PalletId;

use sp_std::convert::TryInto;
/// The module configuration trait.
pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	
    type Balances: Currency<Self::AccountId>;
    type UniqueAssets: UniqueAssets<Self::AccountId>;
    type Lease: Lease<Self::AccountId, NftId<Self>>;
    type PalletId: Get<PalletId<Self>>;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}



decl_storage! {
	trait Store for Module<T: Config> as NftExchange {
        pub NextPlatformId get(fn next_platform_id): u128 = 1;
		pub NextAuctionId get(fn next_auction_id): u128 = 1;

		pub Platforms get(fn platform_by_id): map hasher(blake2_128_concat) u128 => Option<Platform<T::AccountId>>;
		pub Auctions get(fn auction_by_id): map hasher(blake2_128_concat) u128 => Option<Auction<T::AccountId, NftId<T>, BalanceOf<T>>>;
		pub AuctioningNfts get(fn auctioning_nfts): map hasher(blake2_128_concat) NftId<T> => bool;
	}
}

decl_event! {
	pub enum Event<T> where
		AccountId = <T as frame_system::Config>::AccountId,
		NftId = NftId<T>,
		BalanceOf = BalanceOf<T>,
	{
		NewAuction(u128,u128,NftId,AccountId,BalanceOf),
		AuctionBuy(u128,AccountId,u8,BalanceOf),
		NewPlatform(AccountId,u8,AccountId),
		UpdatePlatform(AccountId,u128,u8,AccountId),
		AuctionDone(u128),
		AuctionChangePrice(AccountId,u128,BalanceOf),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		PermissionDenied,
		NotAdmin,
		UnknownType,
		NotFoundData,
		NotNftOwner,
		PercentageOfFeeNotAllowed,
		AuctionAmountNotAllowed,
		MoneyNotEnough,
		NftAuctionDone,
		UnknowPlatform,
		NftAuctioning,
		Invalid
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		
		#[weight = T::WeightInfo::create_platform()]
		fn create_platform(origin,
			admin: T::AccountId,
			percentage_of_fee: u8,
			fee_account: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Self::_create_platform(
				admin,
				percentage_of_fee,
				fee_account,
			)
		}

		#[weight = T::WeightInfo::update_platform()]
		fn update_platform(origin,
			id: u128,
			percentage_of_fee: u8,
			fee_account: T::AccountId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			// check permission
            let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
            ensure!(is_ok == true, Error::<T>::PermissionDenied);
			
			Self::_update_platform(
				sender,
				id,
				percentage_of_fee,
				fee_account,
			)
		}
		#[weight = T::WeightInfo::platform_change_admin()]
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

		#[weight = T::WeightInfo::create_auction()]
		fn create_auction(origin,
			platform_id: u128,
			nft_id: NftId<T>,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let seller = ensure_signed(origin)?;
			
			Self::_create_auction(
				platform_id,
				seller, 
				nft_id,
				amount,
			)
		}

		#[weight = T::WeightInfo::auction_buy()]
		fn auction_buy(origin,
			auction_id: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			Self::_auction_buy(
				auction_id,
				sender, 
			)
		}

		#[weight = T::WeightInfo::auction_buy()]
		fn auction_change_price(origin,
			auction_id: u128,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			Self::_auction_change_price(
				sender, 
				auction_id,
				amount,
			)
		}

		#[weight = T::WeightInfo::auction_done()]
		fn auction_done(origin,
			auction_id: u128,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			Self::_auction_done(
				auction_id,
				sender, 
			)
		}
	}
}

// The main implementation block for the module.
impl<T: Config> NftExchange<T::AccountId, NftId<T>, BalanceOf<T>> for Module<T> {
	// Public immutables
	fn _create_platform(
			admin: T::AccountId,
			percentage_of_fee: u8,
			fee_account: T::AccountId,
	) -> DispatchResult {
		// percentage_of_fee 0~99
		ensure!(percentage_of_fee < 100, Error::<T>::PercentageOfFeeNotAllowed);

		let platform_id = Self::next_platform_id();

		Platforms::<T>::insert(platform_id, Platform {
			id: platform_id,
			admin:admin.clone(),
			percentage_of_fee: percentage_of_fee,
			fee_account: fee_account.clone(),
		});

        NextPlatformId::mutate(|platform_id| *platform_id += 1);

		Self::deposit_event(RawEvent::NewPlatform(
			admin,
			percentage_of_fee,
			fee_account,
		));
		Ok(())
	}

	fn _update_platform(
		admin: T::AccountId,
		id: u128,
		percentage_of_fee: u8,
		fee_account: T::AccountId,
	) -> DispatchResult {
		// percentage_of_fee 0~99
		ensure!(percentage_of_fee < 100, Error::<T>::PercentageOfFeeNotAllowed);

		Platforms::<T>::try_mutate_exists(id, |platform| {
			let _platform = platform.take().ok_or( Error::<T>::NotFoundData)?;
			ensure!(_platform.admin == admin, Error::<T>::NotAdmin);

			*platform = Some(Platform{
				admin: admin.clone(),
				id: id,
				percentage_of_fee: percentage_of_fee,
				fee_account: fee_account.clone(),
			});

			Self::deposit_event(RawEvent::UpdatePlatform(
				admin,
				id,
				percentage_of_fee,
				fee_account,
			));
			Ok(())
		})
	}

	fn _create_auction(
		platform_id: u128,
		seller: T::AccountId,
		nft_id: NftId<T>,
		amount: BalanceOf<T>,
	) -> DispatchResult {
		ensure!(amount > 0u8.try_into().ok().unwrap() , Error::<T>::AuctionAmountNotAllowed);
		ensure!(Platforms::<T>::contains_key(platform_id), Error::<T>::UnknowPlatform);
		
		// check nft owner
		ensure!(T::UniqueAssets::owner_of(&nft_id) == seller, Error::<T>::NotNftOwner);
		let auction_id = Self::next_auction_id();
		
		// check nft_id auctions not exist
		ensure!(!AuctioningNfts::<T>::contains_key(nft_id.clone()), Error::<T>::NftAuctioning);
		

		Auctions::<T>::insert(auction_id, Auction {
			id: auction_id,
			platform_id: platform_id,
			nft_id: nft_id.clone(),
			seller: seller.clone(),
			amount: amount,
			buyer: None,
			percentage_of_fee: 0,
			platform_fee: 0u8.try_into().ok().unwrap(),
		});

        NextAuctionId::mutate(|platform_id| *platform_id += 1);

		AuctioningNfts::<T>::insert(nft_id.clone(), true);

		Self::deposit_event(RawEvent::NewAuction(
			auction_id,
			platform_id,
			nft_id,
			seller,
			amount,
		));
		Ok(())
	}

	fn _auction_buy(
		auction_id: u128,
		buyer: T::AccountId,
	) -> DispatchResult {
		Auctions::<T>::try_mutate_exists(auction_id, |auction| {
			// check info exist
			let _auction = auction.as_mut().ok_or(Error::<T>::NotFoundData)?;

			// check nft_id auctions is exist
			ensure!(AuctioningNfts::<T>::contains_key(_auction.nft_id.clone()), Error::<T>::NftAuctionDone);

			// check balance
			ensure!(T::Balances::free_balance(&buyer) >= _auction.amount, Error::<T>::MoneyNotEnough);


			// double check nft owner
			let owner = T::UniqueAssets::owner_of(&_auction.nft_id);
			// stop auction
			if owner != _auction.seller {
				Self::_auction_done(
					auction_id,
					_auction.seller.clone(), 
				)?
			}
			ensure!(owner == _auction.seller, Error::<T>::NotNftOwner);
			ensure!(buyer.clone() != _auction.seller, Error::<T>::NotNftOwner);
			
			// check platform exist & sender is admin
			let _platform = Platforms::<T>::get(_auction.platform_id).ok_or(Error::<T>::UnknowPlatform)?;
			let amount_u64 = TryInto::<u64>::try_into(_auction.amount).ok().unwrap();
			let percentage_of_fee: u64 = _platform.percentage_of_fee.into();
			let fee: u64 = amount_u64/100u64*percentage_of_fee;

			_auction.buyer = Some(buyer.clone());
			_auction.percentage_of_fee = _platform.percentage_of_fee;
			_auction.platform_fee = fee.try_into().ok().unwrap();

			let seller = _auction.seller.clone();
			T::Balances::transfer(&buyer, &seller, _auction.amount - _auction.platform_fee, ExistenceRequirement::KeepAlive)?;

			if _auction.platform_fee > 0u64.try_into().ok().unwrap() {
				T::Balances::transfer(&buyer, &_platform.fee_account, _auction.platform_fee, ExistenceRequirement::KeepAlive)?;
			}
			
			T::UniqueAssets::transfer(&buyer, &_auction.nft_id)?;
			
			// auction done
			AuctioningNfts::<T>::remove(_auction.nft_id.clone());
			// Auctions::<T>::remove(auction_id);

			Self::deposit_event(RawEvent::AuctionBuy(
				auction_id,
				buyer,
				_auction.percentage_of_fee,
				_auction.platform_fee,
			));
			Ok(())
		})
	}


	fn _auction_change_price(
		sender: T::AccountId,
		auction_id: u128,
		amount: BalanceOf<T>,
	) -> DispatchResult {
		ensure!(amount > 0u8.try_into().ok().unwrap() , Error::<T>::AuctionAmountNotAllowed);

		Auctions::<T>::try_mutate_exists(auction_id, |auction| {
			// check info exist
			let _auction = auction.as_mut().ok_or(Error::<T>::NotFoundData)?;

			// check nft_id auctions is exist
			ensure!(AuctioningNfts::<T>::contains_key(_auction.nft_id.clone()), Error::<T>::NftAuctionDone);

			// double check nft owner
			let owner = T::UniqueAssets::owner_of(&_auction.nft_id);
			// stop auction
			if owner != _auction.seller {
				Self::_auction_done(
					auction_id,
					_auction.seller.clone(), 
				)?
			}
			ensure!(owner == _auction.seller, Error::<T>::NotNftOwner);
			ensure!(sender.clone() == _auction.seller, Error::<T>::Invalid);
			
			// check platform exist & sender is admin
			let _platform = Platforms::<T>::get(_auction.platform_id).ok_or(Error::<T>::UnknowPlatform)?;

			_auction.amount = amount;


			Self::deposit_event(RawEvent::AuctionChangePrice(
				sender,
				auction_id,
				amount,
			));
			Ok(())
		})
	}
	
	// down auction
	fn _auction_done(
		auction_id: u128,
		owner: T::AccountId,
	) -> DispatchResult {
		let _auction = Auctions::<T>::get(auction_id).ok_or(Error::<T>::UnknownType)?;
		ensure!(owner == _auction.seller, Error::<T>::NotAdmin);

		// check nft_id auctions is exist
		ensure!(AuctioningNfts::<T>::contains_key(_auction.nft_id.clone()), Error::<T>::NftAuctionDone);

		// auction done
		AuctioningNfts::<T>::remove(_auction.nft_id.clone());
		// Auctions::<T>::remove(auction_id);

		Self::deposit_event(RawEvent::AuctionDone(
			auction_id,
		));
		Ok(())
	}
}
