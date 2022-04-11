#![cfg_attr(not(feature = "std"), no_std)]

pub mod default_weight;
use default_weight::WeightInfo;


use sp_std::{prelude::*};
use sp_runtime::{RuntimeDebug,RandomNumberGenerator};

use sp_runtime::traits::{Hash, BlakeTwo256};
use codec::{Encode, Decode};
use frame_support::{decl_module, decl_event, decl_storage, decl_error, ensure,
	traits::{Currency,ExistenceRequirement,Get},
	dispatch::{DispatchResult},
};

use frame_system::ensure_signed;

use frame_support::{
    debug,
};

use pallet_nft::UniqueAssets;

use pallet_subgame_assets::{self as SubGameAssets};
use pallet_subgame_assets::{AssetsTrait, AssetsTransfer};

use frame_support::sp_std::convert::{TryInto, TryFrom};

pub type NftId<T> = 
    <<T as Config>::UniqueAssets as UniqueAssets<<T as frame_system::Config>::AccountId>>::AssetId;

pub type BalanceOf<T> =
<<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// package info
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct PackageInfo<AssetId, SGAssetBalance> {
	pub asset_id: AssetId,
	pub amount: SGAssetBalance, // 允許使用的優惠方案(儲值時若無匹配，則1:1上分)
}

/// GameSetting
/// * max_num_of_research: 角色研究盲盒最大上限
/// * research_cooldown_hour: 研究冷卻(小時)
/// * research_consumes_amount_of_sor: 研究需消耗Sor
/// * research_consumes_amount_of_esor_list: 研究需消耗Esor，次數與消耗值對應`Vec`列表
/// * fusion_consumes_amount_of_esor: 融合需消耗Esor
/// * fusion_leve2_probability: 升級R的成功機率
/// * fusion_leve3_probability: 升級SR的成功機率
/// * fusion_leve4_probability: 升級SSR的成功機率
/// * sor_asset_id: Sor資產Id
/// * esor_asset_id: Esor資產Id
/// * package_account: 打包資產暫存地址(警告！請勿更動該帳戶的暫存資產，餘額不足將導致用戶無法unpackage)
/// * research_income_account: 研究所消耗資產收入存入地址
/// * fusion_income_account: 融合所消耗資產收入存入地址

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct SonicRacerSetting<ResearchConsumesAmount, SGAssetBalance, AssetId, AccountId> {
	pub max_num_of_research: u32,
	pub research_cooldown_hour: u64,
	pub research_consumes_amount_of_sor: SGAssetBalance,
	pub research_consumes_amount_of_esor_list: Vec<ResearchConsumesAmount>,
	pub fusion_consumes_amount_of_esor: SGAssetBalance,
	pub fusion_leve2_probability: u32,
	pub fusion_leve3_probability: u32,
	pub fusion_leve4_probability: u32,
	pub sor_asset_id: AssetId,
	pub esor_asset_id: AssetId,
	pub package_account: AccountId,
	pub research_income_account: AccountId,
	pub fusion_income_account: AccountId,
}


#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct ResearchConsumesAmount<SGAssetBalance> {
	pub role_consumes_esor: SGAssetBalance,	//  研究角色需消耗?ESOR
	pub props_consumes_esor: SGAssetBalance,	//  研究道具需消耗?ESOR
}

// ----------------------銷售盲盒-------------------------
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct ApplybuySetting<Moment,Balance,AccountId> {
    pub start_time: Moment,    // 開放購買時間
    pub end_time: Moment,    // 結束購買時間
    pub role_price: Balance,    // 購買金額
    pub props_price: Balance,    // 購買金額
    pub role_sell_count: u32,    // 總銷售數量
    pub role_user_max_buy:u32,   // 限制購買上限
    pub props_sell_count: u32,	
    pub props_user_max_buy: u32,	
	pub income_account: AccountId,	
}

// 盲盒已購數量
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct BlindBoxBoughtCount {
    pub role: u32,
    pub props: u32,
}

// nft typeId
// 盲盒
const NFT_TYPE_ID_BLINDBOX:u8 = 1;
// 角色
const NFT_TYPE_ID_ROLE:u8 = 2;
// 道具
const NFT_TYPE_ID_PROPS:u8 = 3;

// 盲盒typeId
const BLINDBOX_TYPE_ID_ROLE:u8 = 1;
const BLINDBOX_TYPE_ID_PROPS:u8 = 2;


// 角色資料
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct Role<Moment> {
    pub role_id: u32,
	pub research_count: u32,
	pub research_last_time: Moment,
}
// 道具資料
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct Props {
    pub props_id: u32,
	pub level: u32,
}
// 道具等級機率
#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, Default)]
pub struct PropsLevelProbability {
	pub leve1_probability: u32,
	pub leve2_probability: u32,
	pub leve3_probability: u32,
	pub leve4_probability: u32,
}

use sp_runtime::{
	MultiSignature,
	traits::{
		IdentifyAccount, Verify,
	}
};
pub type Signature = MultiSignature;
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The module configuration trait.
pub trait Config: frame_system::Config + SubGameAssets::Config + pallet_timestamp::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type OwnerAddress: Get<Self::AccountId>;
	type PackagePoolAddress: Get<Self::AccountId>;
    type UniqueAssets: UniqueAssets<Self::AccountId>;
    type Assets: AssetsTrait + AssetsTransfer<Self::AccountId, u32>;
    type Balances: Currency<Self::AccountId>;
	/// Weight information for extrinsics in this pallet.
	type WeightInfo: WeightInfo;
}



decl_storage! {
	trait Store for Module<T: Config> as SonicRacer {
		// asset package of nft
		pub AssetPackage get(fn package): map hasher(blake2_128_concat) NftId<T> => PackageInfo<T::AssetId, T::SGAssetBalance>;

		// 遊戲設定
		pub GameSetting get(fn game_setting): SonicRacerSetting<ResearchConsumesAmount<T::SGAssetBalance>, T::SGAssetBalance, T::AssetId, T::AccountId> = SonicRacerSetting{
			max_num_of_research: 5,
			research_cooldown_hour: 24,
			research_consumes_amount_of_sor: 10_u32.into(),
			research_consumes_amount_of_esor_list: vec![
				ResearchConsumesAmount{props_consumes_esor: 100_u32.into() ,role_consumes_esor: 180_u32.into()},
				ResearchConsumesAmount{props_consumes_esor: 225_u32.into() ,role_consumes_esor: 385_u32.into()},
				ResearchConsumesAmount{props_consumes_esor: 500_u32.into() ,role_consumes_esor: 800_u32.into()},
				ResearchConsumesAmount{props_consumes_esor: 1200_u32.into() ,role_consumes_esor: 1800_u32.into()},
				ResearchConsumesAmount{props_consumes_esor: 2735_u32.into() ,role_consumes_esor: 3980_u32.into()},
			],
			fusion_consumes_amount_of_esor: 300_u32.into(),
			fusion_leve2_probability: 28,
			fusion_leve3_probability: 11,
			fusion_leve4_probability: 1,
			sor_asset_id: 1002_u32.into(),
			esor_asset_id: 1003_u32.into(),
			package_account: 
				T::AccountId::decode(&mut &AccountId::from(
					// 3iFsAZv22G5cTuGQzBpFKu58wWiQ7oFJuBcgkMsQPUirm2tu	
					hex_literal::hex!("2c46f54479f019745c0a06d5de89fa7c8b61005233ceb2cd81fcbb7bf334ac23")
				).encode()[..]).unwrap_or_default(),
			research_income_account: 
				T::AccountId::decode(&mut &AccountId::from(
					// 3iFsAZv22G5cTuGQzBpFKu58wWiQ7oFJuBcgkMsQPUirm2tu	
					hex_literal::hex!("2c46f54479f019745c0a06d5de89fa7c8b61005233ceb2cd81fcbb7bf334ac23")
				).encode()[..]).unwrap_or_default(),
			fusion_income_account: 
				T::AccountId::decode(&mut &AccountId::from(
					// 3iFsAZv22G5cTuGQzBpFKu58wWiQ7oFJuBcgkMsQPUirm2tu	
					hex_literal::hex!("2c46f54479f019745c0a06d5de89fa7c8b61005233ceb2cd81fcbb7bf334ac23")
				).encode()[..]).unwrap_or_default(),
		};

		// 紀錄nft類型
		// * 盲盒typeId= 1
		// * 角色typeId= 2
		// * 道具typeId= 3
		// NftToTypeId: map hasher(blake2_128_concat) NftId<T> => u8;

		// 紀錄盲盒類型
		// * 角色typeId= 2
		// * 道具typeId= 3
		BlindBoxTypeId: map hasher(blake2_128_concat) NftId<T> => u8;

		// 道具等級機率
		PropsLevelProbabilitySetting get(fn props_level_probability): PropsLevelProbability = PropsLevelProbability{
			leve1_probability: 670_u32,
			leve2_probability: 260_u32,
			leve3_probability: 65_u32,
			leve4_probability: 5_u32,
		};

		// 道具資料
		NftToProps: map hasher(blake2_128_concat) NftId<T> => Props;
		// 角色資料
		NftToRole: map hasher(blake2_128_concat) NftId<T> => Role<<T as pallet_timestamp::Config>::Moment>;


		// 銷售相關
		
		// 申購設定
		pub ApplybuySet: map hasher(blake2_128_concat) u8 => ApplybuySetting<<T as pallet_timestamp::Config>::Moment, BalanceOf<T>, T::AccountId>;
		pub ApplybuyWhitelistSet: map hasher(blake2_128_concat) u8 => ApplybuySetting<<T as pallet_timestamp::Config>::Moment, BalanceOf<T>, T::AccountId>;

		// 下梯次申購Id
		pub NextApplybuyBatchId get(fn next_applybuy_batch_id): u8 = 1;
		pub NextApplybuyWhitelistBatchId get(fn next_applybuy_whitelist_batch_id): u8 = 1;

		// 每個梯次已購買數量
		pub ApplyboughtCount: map hasher(blake2_128_concat) u8 => BlindBoxBoughtCount;
		pub ApplyboughtWhitelistCount: map hasher(blake2_128_concat) u8 => BlindBoxBoughtCount;

		// 每個梯次user購買數量
		pub UserApplyboughtCount: double_map hasher(blake2_128_concat) u8, hasher(blake2_128_concat) T::AccountId => BlindBoxBoughtCount;
		pub UserApplyboughtWhitelistCount: double_map hasher(blake2_128_concat) u8, hasher(blake2_128_concat) T::AccountId => BlindBoxBoughtCount;

		// 白名單user
		pub ApplybuyWhitelist: map hasher(blake2_128_concat) T::AccountId => bool;
	}
}

decl_event! {
	pub enum Event<T> where
	AccountId = <T as frame_system::Config>::AccountId,
	AssetId = <T as SubGameAssets::Config>::AssetId,
	SGAssetBalance = <T as SubGameAssets::Config>::SGAssetBalance,
	NftId = NftId<T>,
	SonicRacerSetting = SonicRacerSetting<ResearchConsumesAmount<<T as SubGameAssets::Config>::SGAssetBalance>, <T as SubGameAssets::Config>::SGAssetBalance, <T as SubGameAssets::Config>::AssetId, <T as frame_system::Config>::AccountId>,
	Moment = <T as pallet_timestamp::Config>::Moment,
	BalanceOf = BalanceOf<T>,
	{
		// 將Asset打包成nft(打包人, 資產Id, 金額, NftHash)
		AssetPackaged(AccountId, AssetId, SGAssetBalance, NftId),
		// 將package解壓回資產(打包人, 資產Id, 金額, NftHash)
		AssetUnpackaged(AccountId, AssetId, SGAssetBalance, NftId),
		// 遊戲設定
		SetGame(SonicRacerSetting),
		// 批量新增白名單
		AddApplybuyWhitelist(Vec<AccountId>),
		// 批量刪除白名單
		DelApplybuyWhitelist(Vec<AccountId>),
		// 新增申購
		AddApplybuySetting(ApplybuySetting<Moment, BalanceOf, AccountId>),
		// 更新申購(梯次Id, 參數)
		UpdateApplybuySetting(u8, ApplybuySetting<Moment, BalanceOf, AccountId>),
		// 新增申購-白名單
		AddApplybuyWhitelistSetting(ApplybuySetting<Moment, BalanceOf, AccountId>),
		// 更新申購-白名單(梯次Id, 參數)
		UpdateApplybuyWhitelistSetting(u8, ApplybuySetting<Moment, BalanceOf, AccountId>),
		// 購買盲盒(購買人, 梯次Id, type_id(=1(ROLE) or 2(PROPS)), NftHash, 花費SGB)
		UserApplybuy(AccountId, u8, u8, NftId, BalanceOf),
		// 購買盲盒-白名單(購買人, 梯次Id, type_id(=1(ROLE) or 2(PROPS)), NftHash, 花費SGB)
		WhitelistUserApplybuy(AccountId, u8, u8, NftId, BalanceOf),
		// 開啟盲盒(開啟人,盲盒種類,盲盒nfthash,獲得new nfthash, props or role的typeid, props等級 若role=0)
		OpenBlindbox(AccountId, u8, NftId, NftId, u32, u32),
		// 更新Props升等機率
		UpdatePropsLevelProbability(PropsLevelProbability),
		// 道具融合成功(user,道具種類, 道具1 nfthash, 道具1等級, 道具2 nfthash, 道具2等級, 消耗esor, 新nftHash, 新等級)
		PropsFusionSuccess(AccountId,u32,NftId,u32,NftId,u32,SGAssetBalance,NftId,u32),
		// 道具融合失敗(user,道具種類, 道具1 nfthash, 道具1等級, 道具2 nfthash, 道具2等級, 消耗esor, 若成功的新等級)
		PropsFusionFaild(AccountId,u32,NftId,u32,NftId,u32,SGAssetBalance,u32),
		// 研究盲盒(user, role1 nfthash, role1 researched count, role2 nfthash, role2 researched count, blinbox type id, new blinbox nfthash, 消耗的sor, 消耗的esor)
		ResearchBlindbox(AccountId, NftId, u32, NftId, u32, u8, NftId, SGAssetBalance, SGAssetBalance),
	}
}

decl_error! {
	pub enum Error for Module<T: Config> {
		PermissionDenied,
		ApplybuyStarted,
		ApplybuyUnStart,
		UnknownPackage,
		NotFoundAsset,
		NotFoundData,
		UnknownApplybuy,
		ApplybuyEndLessThenStart,
		Overflow,
		UserRolePurchaseLimit,
		UserPropsPurchaseLimit,
		RolePurchaseLimit,
		PropsPurchaseLimit,
		InputDataInvalid,
		// 並未在白名單
		NotWhitelist,
		// 未知的盲盒
		UnknownBlindBox,
		// 未知的道具
		UnknownProps,
		// 未知的道具
		UnknownRole,
		// 非nft Owner
		NotNftOwner,
		// 需要相同道具
		NeedEqureProps,
		// 超過研發上線
		MaxNumOfResearch,
		SorNotEnough,
		EsorNotEnough,
		// 研究冷卻中
		ResearchCooling,
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// asset package相關
		/// 
		#[weight = <T as Config>::WeightInfo::asset_package()]
		fn asset_package(origin,
			asset_id: T::AssetId,
			amount: T::SGAssetBalance,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check asset_id exist
			let asset = SubGameAssets::Asset::<T>::get(asset_id);
			ensure!(asset != None, Error::<T>::NotFoundAsset);
			
			let game_setting = GameSetting::<T>::get();

			SubGameAssets::Module::<T>::_transfer(sender.clone(), asset_id, game_setting.package_account, amount)?;
			
			let nft_hash = T::UniqueAssets::mint(&sender, Vec::new())?;

			AssetPackage::<T>::insert(nft_hash.clone(), PackageInfo {
				asset_id: asset_id.clone(),
				amount: amount,
			});


			Self::deposit_event(RawEvent::AssetPackaged(
				sender,
				asset_id,
				amount,
				nft_hash.clone(),
			));
			Ok(())
			
		}

		#[weight = <T as Config>::WeightInfo::asset_unpackage()]
		fn asset_unpackage(origin,
			nft_hash: NftId<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let package_info = AssetPackage::<T>::try_get(&nft_hash).map_err(|_| Error::<T>::UnknownPackage)?;

			// check nft owner
			let owner = T::UniqueAssets::owner_of(&nft_hash);
			ensure!(owner == sender, Error::<T>::PermissionDenied);


			let game_setting = GameSetting::<T>::get();
			
			SubGameAssets::Module::<T>::_transfer(game_setting.package_account, package_info.asset_id, sender.clone(), package_info.amount)?;
			
			T::UniqueAssets::burn(&nft_hash)?;

			AssetPackage::<T>::remove(nft_hash.clone());


			Self::deposit_event(RawEvent::AssetUnpackaged(
				sender,
				package_info.asset_id,
				package_info.amount,
				nft_hash.clone(),
			));
			Ok(())
			
		}

		#[weight = <T as Config>::WeightInfo::set_game()]
		fn set_game(origin,
			new_game_setting: SonicRacerSetting<ResearchConsumesAmount<T::SGAssetBalance>, T::SGAssetBalance, T::AssetId, T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check admin 
			ensure!(sender == T::OwnerAddress::get(), Error::<T>::PermissionDenied);

			// 檢查研發次數和各次數消耗Esor是否匹配
			ensure!(new_game_setting.max_num_of_research == new_game_setting.research_consumes_amount_of_esor_list.len() as u32, Error::<T>::InputDataInvalid);

			// 融合消耗esor > 0
			ensure!(new_game_setting.fusion_consumes_amount_of_esor > 0.into(), Error::<T>::InputDataInvalid);

			// 研究消耗sor > 0
			ensure!(new_game_setting.research_consumes_amount_of_sor > 0.into(), Error::<T>::InputDataInvalid);

			// sor_asset_id目前不給修改，固定帶入值為1002
			ensure!(new_game_setting.sor_asset_id == 1002.into(), Error::<T>::InputDataInvalid);

			// sor_asset_id目前不給修改，固定帶入值為1003
			ensure!(new_game_setting.esor_asset_id == 1003.into(), Error::<T>::InputDataInvalid);
			
			
			
			// 各項檢查To Do

			GameSetting::<T>::set(new_game_setting.clone());

			Self::deposit_event(RawEvent::SetGame(
				new_game_setting,
			));
			Ok(())
			
		}

		// 銷售盲盒相關

		// 批量新增白名單
		#[weight = <T as Config>::WeightInfo::set_applybuy_whitelist()]
		fn add_whitelist(origin,
			accounts: Vec<T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check admin 
			ensure!(sender == T::OwnerAddress::get(), Error::<T>::PermissionDenied);
			for account in accounts.clone() {
				ApplybuyWhitelist::<T>::insert(account, true);
			}
			

			Self::deposit_event(RawEvent::AddApplybuyWhitelist(
				accounts,
			));
			Ok(())
		}

		// 批量刪除白名單
		#[weight = <T as Config>::WeightInfo::set_applybuy_whitelist()]
		fn del_whitelist(origin,
			accounts: Vec<T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check admin 
			ensure!(sender == T::OwnerAddress::get(), Error::<T>::PermissionDenied);
			for account in accounts.clone() {
				ApplybuyWhitelist::<T>::remove(account);
			}
			

			Self::deposit_event(RawEvent::DelApplybuyWhitelist(
				accounts,
			));
			Ok(())
		}
		 

		// 新增申購設定
		#[weight = <T as Config>::WeightInfo::set_applybuy_whitelist()]
		fn add_applybuy_setting(origin,
			applybuy_setting: ApplybuySetting<<T as pallet_timestamp::Config>::Moment, BalanceOf<T>, T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check admin 
			ensure!(sender == T::OwnerAddress::get(), Error::<T>::PermissionDenied);

			// 檢查資料 start < end
			let new_start_time = applybuy_setting.start_time;
			let new_start_time_ms = TryInto::<u64>::try_into(new_start_time).ok().unwrap(); // convert to u64
			let new_end_time = applybuy_setting.end_time;
			let new_end_time_ms = TryInto::<u64>::try_into(new_end_time).ok().unwrap(); // convert to u64
			ensure!(new_end_time_ms > new_start_time_ms, Error::<T>::ApplybuyEndLessThenStart);

			// 檢查price
			ensure!(applybuy_setting.role_price > 0_u32.into(), Error::<T>::InputDataInvalid);
			ensure!(applybuy_setting.props_price > 0_u32.into(), Error::<T>::InputDataInvalid);
			
			let applybuy_batch_id = Self::next_applybuy_batch_id();

			ApplybuySet::<T>::insert(applybuy_batch_id, applybuy_setting.clone());

			NextApplybuyBatchId::mutate(|applybuy_batch_id| *applybuy_batch_id += 1);

			Self::deposit_event(RawEvent::AddApplybuySetting(
				applybuy_setting,
			));
			Ok(())
		}

		// 更新申購設定(只有now < starttime時，才能修改)
		#[weight = <T as Config>::WeightInfo::set_applybuy_whitelist()]
		fn update_applybuy_setting(origin,
			applybuy_batch_id: u8,
			new_applybuy_setting: ApplybuySetting<<T as pallet_timestamp::Config>::Moment, BalanceOf<T>, T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check admin 
			ensure!(sender == T::OwnerAddress::get(), Error::<T>::PermissionDenied);

			// 檢查price
			ensure!(new_applybuy_setting.role_price > 0_u32.into(), Error::<T>::InputDataInvalid);
			ensure!(new_applybuy_setting.props_price > 0_u32.into(), Error::<T>::InputDataInvalid);
			
			ApplybuySet::<T>::try_mutate_exists(applybuy_batch_id, |applybuy_setting| {
				let _applybuy_setting = applybuy_setting.as_mut().ok_or( Error::<T>::NotFoundData)?;
				
				// 只開放申購前可修改
				let now = pallet_timestamp::Pallet::<T>::get();
				let now_ms = TryInto::<u64>::try_into(now).ok().unwrap(); // convert to u64
				let start_time = _applybuy_setting.start_time;
				let start_time_ms = TryInto::<u64>::try_into(start_time).ok().unwrap(); // convert to u64
				ensure!(now_ms < start_time_ms, Error::<T>::ApplybuyStarted);

				// 檢查資料 start < end
				let new_start_time = new_applybuy_setting.start_time;
				let new_start_time_ms = TryInto::<u64>::try_into(new_start_time).ok().unwrap(); // convert to u64
				let new_end_time = new_applybuy_setting.end_time;
				let new_end_time_ms = TryInto::<u64>::try_into(new_end_time).ok().unwrap(); // convert to u64
				ensure!(new_end_time_ms > new_start_time_ms, Error::<T>::ApplybuyEndLessThenStart);

	
				*applybuy_setting = Some(new_applybuy_setting.clone());
	
				Self::deposit_event(RawEvent::UpdateApplybuySetting(
					applybuy_batch_id,
					new_applybuy_setting,
				));
				Ok(())
			})
		}
		
		// 新增申購設定(白名單)
		#[weight = <T as Config>::WeightInfo::set_applybuy_whitelist()]
		fn add_applybuy_whitelist_setting(origin,
			applybuy_setting: ApplybuySetting<<T as pallet_timestamp::Config>::Moment, BalanceOf<T>, T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check admin 
			ensure!(sender == T::OwnerAddress::get(), Error::<T>::PermissionDenied);

			// 檢查price
			ensure!(applybuy_setting.role_price > 0_u32.into(), Error::<T>::InputDataInvalid);
			ensure!(applybuy_setting.props_price > 0_u32.into(), Error::<T>::InputDataInvalid);
			
			// 檢查資料 start < end
			let new_start_time = applybuy_setting.start_time;
			let new_start_time_ms = TryInto::<u64>::try_into(new_start_time).ok().unwrap(); // convert to u64
			let new_end_time = applybuy_setting.end_time;
			let new_end_time_ms = TryInto::<u64>::try_into(new_end_time).ok().unwrap(); // convert to u64
			ensure!(new_end_time_ms > new_start_time_ms, Error::<T>::ApplybuyEndLessThenStart);
			
			let applybuy_batch_id = Self::next_applybuy_whitelist_batch_id();

			ApplybuyWhitelistSet::<T>::insert(applybuy_batch_id, applybuy_setting.clone());

			NextApplybuyWhitelistBatchId::mutate(|applybuy_batch_id| *applybuy_batch_id += 1);

			Self::deposit_event(RawEvent::AddApplybuyWhitelistSetting(
				applybuy_setting,
			));
			Ok(())
		}

		// 更新申購設定(只有now < starttime時，才能修改)(白名單)
		#[weight = <T as Config>::WeightInfo::set_applybuy_whitelist()]
		fn update_applybuy_whitelist_setting(origin,
			applybuy_batch_id: u8,
			new_applybuy_setting: ApplybuySetting<<T as pallet_timestamp::Config>::Moment, BalanceOf<T>, T::AccountId>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check admin 
			ensure!(sender == T::OwnerAddress::get(), Error::<T>::PermissionDenied);

			// 檢查price
			ensure!(new_applybuy_setting.role_price > 0_u32.into(), Error::<T>::InputDataInvalid);
			ensure!(new_applybuy_setting.props_price > 0_u32.into(), Error::<T>::InputDataInvalid);
			
			ApplybuyWhitelistSet::<T>::try_mutate_exists(applybuy_batch_id, |applybuy_setting| {
				let _applybuy_setting = applybuy_setting.as_mut().ok_or( Error::<T>::NotFoundData)?;
				
				// 只開放申購前可修改
				let now = pallet_timestamp::Pallet::<T>::get();
				let now_ms = TryInto::<u64>::try_into(now).ok().unwrap(); // convert to u64
				let start_time = _applybuy_setting.start_time;
				let start_time_ms = TryInto::<u64>::try_into(start_time).ok().unwrap(); // convert to u64
				ensure!(now_ms < start_time_ms, Error::<T>::ApplybuyStarted);

				// 檢查資料 start < end
				let new_start_time = new_applybuy_setting.start_time;
				let new_start_time_ms = TryInto::<u64>::try_into(new_start_time).ok().unwrap(); // convert to u64
				let new_end_time = new_applybuy_setting.end_time;
				let new_end_time_ms = TryInto::<u64>::try_into(new_end_time).ok().unwrap(); // convert to u64
				ensure!(new_end_time_ms > new_start_time_ms, Error::<T>::ApplybuyEndLessThenStart);
	
				*applybuy_setting = Some(new_applybuy_setting.clone());
	
				Self::deposit_event(RawEvent::UpdateApplybuyWhitelistSetting(
					applybuy_batch_id,
					new_applybuy_setting,
				));
				Ok(())
			})
		}

		// 購買盲盒
		#[weight = <T as Config>::WeightInfo::user_applybuy()]
		fn user_applybuy(origin,
			applybuy_batch_id: u8,
			blindbox_type_id: u8,
			quantity: u32,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// 一次不能買超過100(保護chain)
			ensure!(quantity <= 100 , Error::<T>::InputDataInvalid);

			// 檢查blindbox_type_id正確
			ensure!(blindbox_type_id == BLINDBOX_TYPE_ID_PROPS || blindbox_type_id == BLINDBOX_TYPE_ID_ROLE , Error::<T>::NotFoundData);

			// check applybuy_batch exist
			let _applybuy_setting = ApplybuySet::<T>::try_get(&applybuy_batch_id).map_err(|_| Error::<T>::UnknownApplybuy)?;

			// 是否為申購期間
			let now = pallet_timestamp::Pallet::<T>::get();
			let now_ms = TryInto::<u64>::try_into(now).ok().unwrap(); // convert to u64
			let start_time = _applybuy_setting.start_time;
			let start_time_ms = TryInto::<u64>::try_into(start_time).ok().unwrap(); // convert to u64
			let end_time = _applybuy_setting.end_time;
			let end_time_ms = TryInto::<u64>::try_into(end_time).ok().unwrap(); // convert to u64
			ensure!(now_ms >= start_time_ms, Error::<T>::ApplybuyUnStart);
			ensure!(now_ms < end_time_ms, Error::<T>::ApplybuyUnStart);

			// 檢查user購買數量
			let mut _user_applybought_count = UserApplyboughtCount::<T>::get(applybuy_batch_id, &sender);
			if blindbox_type_id == BLINDBOX_TYPE_ID_ROLE {
				_user_applybought_count.role = _user_applybought_count.role.checked_add(quantity).ok_or(Error::<T>::Overflow)?;
				ensure!(_user_applybought_count.role <= _applybuy_setting.role_user_max_buy, Error::<T>::UserRolePurchaseLimit);
			}else if blindbox_type_id == BLINDBOX_TYPE_ID_PROPS {
				_user_applybought_count.props = _user_applybought_count.props.checked_add(quantity).ok_or(Error::<T>::Overflow)?;
				ensure!(_user_applybought_count.props <= _applybuy_setting.props_user_max_buy, Error::<T>::UserPropsPurchaseLimit);
			}

			// 檢查全網購買數量
			let mut _applybought_count = ApplyboughtCount::get(&applybuy_batch_id);
			if blindbox_type_id == BLINDBOX_TYPE_ID_ROLE {
				_applybought_count.role = _applybought_count.role.checked_add(quantity).ok_or(Error::<T>::Overflow)?;
				ensure!(_applybought_count.role <= _applybuy_setting.role_sell_count, Error::<T>::RolePurchaseLimit);
			}else if blindbox_type_id == BLINDBOX_TYPE_ID_PROPS{
				_applybought_count.props = _applybought_count.props.checked_add(quantity).ok_or(Error::<T>::Overflow)?;
				ensure!(_applybought_count.props <= _applybuy_setting.props_sell_count, Error::<T>::PropsPurchaseLimit);
			}
			
			// 計算所需SGB
			let mut price: BalanceOf<T>;
			if blindbox_type_id == BLINDBOX_TYPE_ID_ROLE {
				price = _applybuy_setting.role_price
			}else{
				price = _applybuy_setting.props_price
			}
			let amount = price * quantity.into();
			T::Balances::transfer(&sender, &_applybuy_setting.income_account, amount, ExistenceRequirement::KeepAlive)?;

			// update storage
			ApplyboughtCount::mutate(applybuy_batch_id, |count_data| *count_data = _applybought_count);
			UserApplyboughtCount::<T>::mutate(applybuy_batch_id, &sender, |count_data| *count_data = _user_applybought_count);
			
			for _i in 1..(quantity+1)
			{  
				let nft_id = T::UniqueAssets::mint(&sender,Vec::new())?;
				// update storage
				// NftToTypeId::<T>::insert(&nft_id, NFT_TYPE_ID_BLINDBOX);
				BlindBoxTypeId::<T>::insert(&nft_id, blindbox_type_id);
				Self::deposit_event(RawEvent::UserApplybuy(
					sender.clone(),
					applybuy_batch_id,
					blindbox_type_id,
					nft_id,
					price,
				));
			}   
			Ok(())
		}

		// 購買白名單盲盒
		#[weight = <T as Config>::WeightInfo::user_applybuy()]
		fn whitelist_user_applybuy(origin,
			applybuy_batch_id: u8,
			blindbox_type_id: u8,
			quantity: u32,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// 一次不能買超過100(保護chain)
			ensure!(quantity <= 100 , Error::<T>::InputDataInvalid);

			// 檢查blindbox_type_id正確
			ensure!(blindbox_type_id == BLINDBOX_TYPE_ID_PROPS || blindbox_type_id == BLINDBOX_TYPE_ID_ROLE , Error::<T>::NotFoundData);

			// check applybuy_batch exist
			let _applybuy_setting = ApplybuyWhitelistSet::<T>::try_get(&applybuy_batch_id).map_err(|_| Error::<T>::UnknownApplybuy)?;

			// 檢查是否為白名單
			ensure!(ApplybuyWhitelist::<T>::contains_key(&sender) , Error::<T>::NotWhitelist);

			// 是否為申購期間
			let now = pallet_timestamp::Pallet::<T>::get();
			let now_ms = TryInto::<u64>::try_into(now).ok().unwrap(); // convert to u64
			let start_time = _applybuy_setting.start_time;
			let start_time_ms = TryInto::<u64>::try_into(start_time).ok().unwrap(); // convert to u64
			let end_time = _applybuy_setting.end_time;
			let end_time_ms = TryInto::<u64>::try_into(end_time).ok().unwrap(); // convert to u64
			ensure!(now_ms >= start_time_ms, Error::<T>::ApplybuyUnStart);
			ensure!(now_ms < end_time_ms, Error::<T>::ApplybuyUnStart);

			// 檢查user購買數量
			let mut _user_applybought_count = UserApplyboughtWhitelistCount::<T>::get(applybuy_batch_id, &sender);
			if blindbox_type_id == BLINDBOX_TYPE_ID_ROLE {
				_user_applybought_count.role = _user_applybought_count.role.checked_add(quantity).ok_or(Error::<T>::Overflow)?;
				ensure!(_user_applybought_count.role <= _applybuy_setting.role_user_max_buy, Error::<T>::UserRolePurchaseLimit);
			}else if blindbox_type_id == BLINDBOX_TYPE_ID_PROPS {
				_user_applybought_count.props = _user_applybought_count.props.checked_add(quantity).ok_or(Error::<T>::Overflow)?;
				ensure!(_user_applybought_count.props <= _applybuy_setting.props_user_max_buy, Error::<T>::UserPropsPurchaseLimit);
			}

			// 檢查全網購買數量
			let mut _applybought_count = ApplyboughtWhitelistCount::get(&applybuy_batch_id);
			if blindbox_type_id == BLINDBOX_TYPE_ID_ROLE {
				_applybought_count.role = _applybought_count.role.checked_add(quantity).ok_or(Error::<T>::Overflow)?;
				ensure!(_applybought_count.role <= _applybuy_setting.role_sell_count, Error::<T>::RolePurchaseLimit);
			}else if blindbox_type_id == BLINDBOX_TYPE_ID_PROPS{
				_applybought_count.props = _applybought_count.props.checked_add(quantity).ok_or(Error::<T>::Overflow)?;
				ensure!(_applybought_count.props <= _applybuy_setting.props_sell_count, Error::<T>::PropsPurchaseLimit);
			}
			
			// 計算所需SGB
			let mut price: BalanceOf<T>;
			if blindbox_type_id == BLINDBOX_TYPE_ID_ROLE {
				price = _applybuy_setting.role_price
			}else{
				price = _applybuy_setting.props_price
			}
			let amount = price * quantity.into();
			T::Balances::transfer(&sender, &_applybuy_setting.income_account, amount, ExistenceRequirement::KeepAlive)?;

			// update storage
			ApplyboughtWhitelistCount::mutate(applybuy_batch_id, |count_data| *count_data = _applybought_count);
			UserApplyboughtWhitelistCount::<T>::mutate(applybuy_batch_id, &sender, |count_data| *count_data = _user_applybought_count);
			
			for _i in 1..(quantity+1)
			{  
				let nft_id = T::UniqueAssets::mint(&sender,Vec::new())?;
				// update storage
				// NftToTypeId::<T>::insert(&nft_id, NFT_TYPE_ID_BLINDBOX);
				BlindBoxTypeId::<T>::insert(&nft_id, blindbox_type_id);
				Self::deposit_event(RawEvent::WhitelistUserApplybuy(
					sender.clone(),
					applybuy_batch_id,
					blindbox_type_id,
					nft_id,
					price,
				));
			}   
			Ok(())
		}

		// 開啟盲盒
		#[weight = <T as Config>::WeightInfo::open_blindbox()]
		fn open_blindbox(origin,
			nfthash: NftId<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let _type_id = BlindBoxTypeId::<T>::try_get(&nfthash).map_err(|_| Error::<T>::UnknownBlindBox)?;

			// 檢查blindbox_type_id是否正確
			ensure!(_type_id == BLINDBOX_TYPE_ID_PROPS || _type_id == BLINDBOX_TYPE_ID_ROLE , Error::<T>::UnknownBlindBox);
			
			// 檢查owner
			ensure!(T::UniqueAssets::owner_of(&nfthash) == sender, Error::<T>::NotNftOwner);
			 
			let props_level_probability = PropsLevelProbabilitySetting::get();
			
			let total_probability = props_level_probability.leve1_probability + props_level_probability.leve2_probability + props_level_probability.leve3_probability + props_level_probability.leve4_probability;

			// 道具盲盒
			if _type_id == BLINDBOX_TYPE_ID_PROPS {
				// 取得道具種類random 1~6
				let max = 6 + 1;
				let min = 1;
				let props_id = Self::_range_random(min,max);


				// 取得道具等級
				let mut props_level = 1;
				let max = total_probability + 1;
				let min = 0;
				let ran_num = Self::_range_random(min,max);
				if ran_num <= props_level_probability.leve1_probability {
					props_level = 1
				}else if ran_num <= (props_level_probability.leve1_probability + props_level_probability.leve2_probability) {
					props_level = 2
				}else if ran_num <= (props_level_probability.leve1_probability + props_level_probability.leve2_probability + props_level_probability.leve3_probability) {
					props_level = 3
				}else if ran_num <= total_probability {
					props_level = 4
				}

				let nft_id = T::UniqueAssets::mint(&sender, Vec::new())?;
				BlindBoxTypeId::<T>::remove(&nfthash);
				T::UniqueAssets::burn(&nfthash)?;

				NftToProps::<T>::insert(&nft_id, Props{
					props_id: props_id,
					level: props_level,
				});
				Self::deposit_event(RawEvent::OpenBlindbox(
					sender.clone(),
					_type_id,
					nfthash,
					nft_id,
					props_id,
					props_level,
				));
				
			}else{
				// 取得道具種類random 1~6
				let max = 6 + 1;
				let min = 1;
				let role_id = Self::_range_random(min,max);


				let nft_id = T::UniqueAssets::mint(&sender, Vec::new())?;
				BlindBoxTypeId::<T>::remove(&nfthash);
				T::UniqueAssets::burn(&nfthash)?;

				NftToRole::<T>::insert(&nft_id, Role{
					role_id: role_id,
					research_count: 0,
					research_last_time: TryInto::<T::Moment>::try_into(0_u64).ok().unwrap(),
				});

				Self::deposit_event(RawEvent::OpenBlindbox(
					sender.clone(),
					_type_id,
					nfthash,
					nft_id,
					role_id,
					0,
				));
			}
			
			Ok(())
		}

		// 設定道具等級機率(機率加總需=1000)
		#[weight = <T as Config>::WeightInfo::set_applybuy_whitelist()]
		fn update_props_level_probability(origin,
			new_setting: PropsLevelProbability,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// check admin 
			ensure!(sender == T::OwnerAddress::get(), Error::<T>::PermissionDenied);

			// 檢查機率加總需=1000 
			let total = new_setting.leve1_probability + 
			new_setting.leve2_probability + 
			new_setting.leve3_probability + 
			new_setting.leve4_probability;

			ensure!(total == 1000, Error::<T>::InputDataInvalid);
			

			PropsLevelProbabilitySetting::set(new_setting.clone());

			Self::deposit_event(RawEvent::UpdatePropsLevelProbability(
				new_setting,
			));
			Ok(())
		}

		// 融合
		#[weight = <T as Config>::WeightInfo::props_fusion()]
		fn props_fusion(origin,
			nfthash1: NftId<T>,
			nfthash2: NftId<T>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			
			let prop1 = NftToProps::<T>::try_get(&nfthash1).map_err(|_| Error::<T>::UnknownProps)?;
			let prop2 = NftToProps::<T>::try_get(&nfthash2).map_err(|_| Error::<T>::UnknownProps)?;

			// 檢查是否為同類型道具
			ensure!(prop1.props_id == prop2.props_id, Error::<T>::NeedEqureProps);
			let props_id = prop1.props_id;
			
			// 檢查owner
			ensure!(T::UniqueAssets::owner_of(&nfthash1) == sender, Error::<T>::NotNftOwner);
			ensure!(T::UniqueAssets::owner_of(&nfthash2) == sender, Error::<T>::NotNftOwner);

			// 計算融合等級(2者最低等升級)
			let mut new_level = 2;
			if prop1.level <= prop2.level {
				new_level = prop1.level + 1;
			}else{
				new_level = prop2.level + 1;
			}

			let game_setting = GameSetting::<T>::get();

			// 消耗esor
			let need_esor = game_setting.fusion_consumes_amount_of_esor;

			// 機率判斷
			let mut level_up_probability = 0;
			if new_level == 2 {
				level_up_probability = game_setting.fusion_leve2_probability;
			}else if new_level == 3 {
				level_up_probability = game_setting.fusion_leve3_probability;
			}else if new_level == 4 {
				level_up_probability = game_setting.fusion_leve4_probability;
			}
			
			let max = 100 + 1;
			let min = 1;
			let random_num = Self::_range_random(min,max);
			if random_num <= level_up_probability {
				// 合成成功
				SubGameAssets::Module::<T>::_transfer(sender.clone(), game_setting.esor_asset_id, game_setting.fusion_income_account , need_esor)?;
				
				NftToProps::<T>::remove(&nfthash1);
				NftToProps::<T>::remove(&nfthash2);
				T::UniqueAssets::burn(&nfthash1)?;
				T::UniqueAssets::burn(&nfthash2)?;

				let new_nft_id = T::UniqueAssets::mint(&sender, Vec::new())?;
				NftToProps::<T>::insert(&new_nft_id, Props{
					props_id: props_id,
					level: new_level,
				});

				Self::deposit_event(RawEvent::PropsFusionSuccess(
					sender,
					props_id,
					nfthash1,
					prop1.level,
					nfthash2,
					prop2.level,
					need_esor,
					new_nft_id,
					new_level,
				));
			}else{
				// 合成失敗
				SubGameAssets::Module::<T>::_transfer(sender.clone(), game_setting.esor_asset_id, game_setting.fusion_income_account , need_esor)?;
				
				Self::deposit_event(RawEvent::PropsFusionFaild(
					sender,
					props_id,
					nfthash1,
					prop1.level,
					nfthash2,
					prop2.level,
					need_esor,
					new_level,
				));
			}
			Ok(())
		}
		// 研究盲盒
		#[weight = <T as Config>::WeightInfo::props_fusion()]
		fn research_blindbox(origin,
			nfthash1: NftId<T>,
			nfthash2: NftId<T>,
			type_id: u8,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// 檢查type_id合法（道具or角色）
			ensure!(type_id == NFT_TYPE_ID_ROLE || type_id == NFT_TYPE_ID_PROPS, Error::<T>::InputDataInvalid);
			
			let role1 = NftToRole::<T>::try_get(&nfthash1).map_err(|_| Error::<T>::UnknownRole)?;
			let role2 = NftToRole::<T>::try_get(&nfthash2).map_err(|_| Error::<T>::UnknownRole)?;

			// 檢查owner
			ensure!(T::UniqueAssets::owner_of(&nfthash1) == sender, Error::<T>::NotNftOwner);
			ensure!(T::UniqueAssets::owner_of(&nfthash2) == sender, Error::<T>::NotNftOwner);
			
			let game_setting = GameSetting::<T>::get();

			// 檢查研發上線
			ensure!(role1.research_count < game_setting.max_num_of_research, Error::<T>::MaxNumOfResearch);
			ensure!(role2.research_count < game_setting.max_num_of_research, Error::<T>::MaxNumOfResearch);
			
			// 檢查研發冷卻
			
			let now = pallet_timestamp::Pallet::<T>::get();
			let now_ms = TryInto::<u64>::try_into(now).ok().unwrap(); // convert to u64

			let role1_last_time = role1.research_last_time;
			let role1_last_time_ms = TryInto::<u64>::try_into(role1_last_time).ok().unwrap(); // convert to u64
			let role2_last_time = role2.research_last_time;
			let role2_last_time_ms = TryInto::<u64>::try_into(role2_last_time).ok().unwrap(); // convert to u64

			// add N hour
			let n_hour_ms = u64::try_from(chrono::Duration::hours(game_setting.research_cooldown_hour as i64).num_milliseconds()).ok().unwrap();
			
			let role1_expires_at_ms = role1_last_time_ms + n_hour_ms;
			let role2_expires_at_ms = role2_last_time_ms + n_hour_ms;
			
			ensure!(now_ms >= role1_expires_at_ms, Error::<T>::ResearchCooling);
			ensure!(now_ms >= role2_expires_at_ms, Error::<T>::ResearchCooling);

			// 取得消耗sor
			let need_sor = game_setting.research_consumes_amount_of_sor;
			// 取得消耗esor
			let role1_research_esor = game_setting.research_consumes_amount_of_esor_list[role1.research_count as usize].clone();
			let role2_research_esor = game_setting.research_consumes_amount_of_esor_list[role2.research_count as usize].clone();
			let need_esor: T::SGAssetBalance;
			if type_id == NFT_TYPE_ID_ROLE {
				need_esor = role1_research_esor.role_consumes_esor + role2_research_esor.role_consumes_esor;
			}else{
				need_esor = role1_research_esor.props_consumes_esor + role2_research_esor.props_consumes_esor;
			}

			// 檢查Sor足夠
			let sor_balance = SubGameAssets::Module::<T>::balance(game_setting.sor_asset_id, sender.clone());
			ensure!(sor_balance >= need_sor, Error::<T>::SorNotEnough);

			// 檢查Esor足夠
			let esor_balance = SubGameAssets::Module::<T>::balance(game_setting.esor_asset_id, sender.clone());
			ensure!(esor_balance >= need_esor, Error::<T>::EsorNotEnough);
			
			// 開始研究
			SubGameAssets::Module::<T>::_transfer(sender.clone(), game_setting.esor_asset_id, game_setting.research_income_account.clone(), need_esor)?;
			SubGameAssets::Module::<T>::_transfer(sender.clone(), game_setting.sor_asset_id, game_setting.research_income_account.clone(), need_sor)?;
			
			
			// 更新研究次數和研究時間
			let _now = pallet_timestamp::Pallet::<T>::get();
			NftToRole::<T>::mutate(&nfthash1, |_role| {
				_role.research_last_time = _now;
				_role.research_count += 1;
			});
			NftToRole::<T>::mutate(&nfthash2, |_role| {
				_role.research_last_time = _now;
				_role.research_count += 1;
			});

			let nft_id = T::UniqueAssets::mint(&sender, Vec::new())?;
			BlindBoxTypeId::<T>::insert(&nft_id, type_id);

			Self::deposit_event(RawEvent::ResearchBlindbox(
				sender,
				nfthash1,
				role1.research_count+1,
				nfthash2,
				role2.research_count+1,
				type_id,
				nft_id,
				need_sor,
				need_esor,
			));
			Ok(())
		}
		
	}
}

// The main implementation block for the module.
impl<T: Config> Module<T> {
    
	/// ran
	fn _range_random(
		min: u32,
		max: u32,
	) -> u32 {
		let result: &u32;
		let mut range = Vec::new();


		let now = pallet_timestamp::Pallet::<T>::get();
		let random_seed = BlakeTwo256::hash(&(now.encode()));
		let mut rng = RandomNumberGenerator::<BlakeTwo256>::new(random_seed);
		for n in min..max{
			range.push(n);
		}
		let result = rng.pick_item(&range).unwrap();
		*result
	}

}
