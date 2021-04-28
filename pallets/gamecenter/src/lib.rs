// 可開局及下注、查詢指定template 目前可下注遊戲和歷史遊戲
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{debug, decl_module, decl_storage, decl_event, decl_error, dispatch, ensure,
	weights::Weight, 
	Parameter,
	dispatch::{
		Vec,
	},};
use sp_runtime::{DispatchError, traits::{AtLeast32Bit,Bounded}};
use frame_system::ensure_signed;
use codec::{Encode, Decode};

extern crate alloc;
use alloc::{format, str, string::*};


use pallet_gametemplates_guess_hash::{GuessHashTrait, GuessHashFunc};

mod default_weight;

#[derive(Encode, Decode, Default)]
pub struct GameInstance <GameInstanceID, Owner, DrawBlockNumber, Chips> {
	// 局號
	game_instance_id: GameInstanceID,
    // 開局人
	owner: Owner,
	// 賭注區塊num
	bet_block_number: DrawBlockNumber,	
	// 籌碼獎池
    chips_pool: Chips,
	// 遊戲是否結束
	game_over: bool,
}

// 定義型態
pub type GameInstanceID = u32;

pub trait WeightInfo {
	fn play_game() -> Weight;
	fn create_game() -> Weight;
	fn on_finalize(count: u32) -> Weight;
}
pub trait Config: frame_system::Config{
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type WeightInfo: WeightInfo;
	type GuessHash: GuessHashTrait + GuessHashFunc<Self::AccountId, GameInstanceID, u128>;
}

decl_storage! {
	trait Store for Module<T: Config> as GameCenterModule {
		pub CurrentGameinstances get(fn get_current_gameinstances): map hasher(blake2_128_concat) u32=> Vec<GameInstance<GameInstanceID, T::AccountId, T::BlockNumber, u128>>;
		pub HistoryGameinstances get(fn get_history_gameinstances): map hasher(blake2_128_concat) u32=> Vec<GameInstance<GameInstanceID, T::AccountId, T::BlockNumber, u128>>;
		pub PlayMap get(fn get_playmap): map hasher(blake2_128_concat) T::AccountId=> Vec<u32>;
		DrawMap get(fn draw_map): map hasher(blake2_128_concat) T::BlockNumber => Vec<GameInstanceID>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, BlockNumber = <T as frame_system::Config>::BlockNumber {
		//	開局（莊家, GameIndex, 獎池金額, 下注區塊）
		CreateGame(AccountId, BlockNumber),
		//	下注（玩家, 遊戲ID, 下注金額, 1:單 or 2:雙, 下注id）
		Bet(AccountId, u32),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		NoneValue,
		StorageOverflow,
		GameCountOverflow,
		GameIsNotExist,
		GameModeIsNotExist,
		BalanceNotEnough,
		TransferError,
		BetChipsLimitError,	// 下注籌碼到達上限
		GameOver,	// 遊戲結束
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// 只要有用到Error就要這行
		type Error = Error::<T>;

		// 只要有用到Event就要這行
		fn deposit_event() = default;
		
		#[weight = T::WeightInfo::create_game()]
		pub fn create_game(origin, template_id: u32, bet_next_few_block: u32, amount: u128) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			let game_id = T::GuessHash::create_game(&sender, bet_next_few_block, amount)?;


			let block_number = <frame_system::Module<T>>::block_number(); 
			// 賭注的區塊
			let bet_block_number = block_number + bet_next_few_block.into();
			
			// 新增遊戲局
			let gameinstances = GameInstance{
				game_instance_id: game_id,
				owner: sender.clone(),
				bet_block_number: bet_block_number,	
				chips_pool: amount,
				game_over: false,
			};
			let mut current_game_list = CurrentGameinstances::<T>::get(template_id);	
			let index = current_game_list.len();	
			current_game_list.insert(index, gameinstances);	// insert新紀錄
			CurrentGameinstances::<T>::insert(&template_id, current_game_list);	
			
			// 更新開獎區塊
			let mut drap_map = DrawMap::<T>::get(bet_block_number);
			drap_map.insert(drap_map.len(), game_id);
			DrawMap::<T>::insert(bet_block_number, drap_map);	

			Ok(())
		}
		
		#[weight = T::WeightInfo::play_game()]
		pub fn play_game(origin, game_id: u32, amount: u128, game_mode: u8) -> dispatch::DispatchResult {
			let sender = ensure_signed(origin)?;
			T::GuessHash::bet(&sender, game_id , amount, game_mode)?;
			Ok(())
		}

		//此函數必須返回on_initialize和on_finalize消耗的權重。
		fn on_initialize(now: T::BlockNumber) -> Weight {
			T::WeightInfo::on_finalize(2u32)
		}

		fn on_finalize(now: T::BlockNumber) {
			let game_id_list = Self::draw_map(now);
			// 準備開獎
			if game_id_list.len() > 0 { 
				for game_id in game_id_list {
					Self::game_over(1, game_id);
				}
			}
		}
	}
}

impl<T: Config> Module<T> {
	fn game_over(template_id: u32, game_instance_id: u32)  -> dispatch::DispatchResult {
		let for_current_game_list = CurrentGameinstances::<T>::get(template_id);
		let mut current_game_list = CurrentGameinstances::<T>::get(template_id);
		let mut history_game_list = HistoryGameinstances::<T>::get(template_id);	
		let mut remove_game_instance_id: usize =  999999999;
		let mut tmp_data: Option<GameInstance<GameInstanceID, T::AccountId, T::BlockNumber, u128>> = None;
		for (k, v) in for_current_game_list.into_iter().enumerate() {
			if v.game_instance_id == game_instance_id {
				remove_game_instance_id = k;
				tmp_data = Some(v);
			}
		}
		if tmp_data.is_some() {
			current_game_list.remove(remove_game_instance_id);
			CurrentGameinstances::<T>::insert(template_id, current_game_list);
			history_game_list.insert(history_game_list.len(), tmp_data.unwrap());
			HistoryGameinstances::<T>::insert(template_id, history_game_list);
		}
		Ok(())
	}
}