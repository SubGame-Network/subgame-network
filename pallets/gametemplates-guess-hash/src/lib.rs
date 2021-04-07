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

// 引入chips特徵
use pallet_chips::{ChipsTrait, ChipsTransfer};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod default_weight;

#[derive(Encode, Decode, Default)]
pub struct GameInfo <Owner, BlockNumber, DrawBlockNumber, Amount> {
    // 開局人
	owner: Owner,
	// 區塊num
    block_number: BlockNumber,
	// 賭注區塊num
	bet_block_number: DrawBlockNumber,	
	// 獎池金額（下注總金額不能大於獎池金額）
    amount: Amount
}
#[derive(Encode, Decode, Default)]
#[derive(Debug)]
pub struct BetInfo <Account, GameIndex, Amount, GameMode> {
	// bet user
    user: Account,
	// game index
    game_id: GameIndex,
	// bet amount
    amount: Amount,
	// game mode(1 or 2)
	game_mode: GameMode
}
pub trait WeightInfo {
	fn create_game() -> Weight;
	fn bet() -> Weight;
	fn on_finalize(count: u32) -> Weight;
}
pub trait Config: frame_system::Config{
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type GameIndex: Parameter + AtLeast32Bit + Bounded + Default + Copy;
	type WeightInfo: WeightInfo;
	type Chips: ChipsTrait + ChipsTransfer<Self::AccountId>;
}

type ChipBalance<T> = <<T as Config>::Chips as pallet_chips::ChipsTrait>::ChipBalance;

// 定義遊戲模式
// 猜單數 || 猜雙數
pub type GameMode = u8;
pub const GameModeIsSingle: GameMode = 1;
pub const GameModeIsDouble: GameMode = 2;

decl_storage! {
	trait Store for Module<T: Config> as GameGuessHashModule {
		pub GameList get(fn game_list): map hasher(blake2_128_concat)  T::GameIndex => GameInfo<T::AccountId, T::BlockNumber, T::BlockNumber, ChipBalance<T>>;
		pub BetList get(fn bet_list): map hasher(blake2_128_concat)  T::GameIndex => Vec<BetInfo<T::AccountId, T::GameIndex, ChipBalance<T>, GameMode>>;
		pub GameCount get(fn game_count): T::GameIndex;
		pub DrawMap get(fn draw_map): map hasher(blake2_128_concat) T::BlockNumber => Option<T::GameIndex>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId, ChipBalance = ChipBalance<T>, GameIndex = <T as Config>::GameIndex , BlockNumber = <T as frame_system::Config>::BlockNumber , BlockHash = <T as frame_system::Config>::Hash {
		//	開局（莊家, GameIndex, 獎池金額, 下注區塊）
		CreateGame(AccountId, GameIndex, ChipBalance, BlockNumber),
		//	下注（玩家, 遊戲ID, 下注金額, 1:單 or 2:雙, 下注id）
		Bet(AccountId, GameIndex, ChipBalance, GameMode, u32),
		//	玩家結算獲獎金額（玩家, 遊戲ID, 贏得金額, 下注ID, 遊戲結果（1:單 or 2:雙）, 開獎的Block Hash）
		BettorResult(AccountId, GameIndex, ChipBalance, u32, GameMode, BlockHash),
		//	遊戲結束（莊家, 遊戲ID, 莊家拿到的總金額, 遊戲結果（1:單 or 2:雙）, 開獎的Block Hash）
		GameOver(AccountId, GameIndex, ChipBalance, GameMode, BlockHash),
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
		BetAmountLimitError,	// 下注金額到達上限
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
		pub fn create_game(origin, bet_next_few_block: u32, amount: ChipBalance<T>) -> dispatch::DispatchResult {
			// 開局人
			let sender = ensure_signed(origin)?;
			// 當前交易的block number
			let block_number = <frame_system::Module<T>>::block_number(); 
			// 取得新遊戲的Index
			let game_id = Self::next_game_id()?;

			// 賭注的區塊
			let bet_block_number = block_number + bet_next_few_block.into();
			let game_info = GameInfo{
				owner: sender.clone(),
				block_number: block_number,
				bet_block_number: bet_block_number,
				amount: amount
			};
			<GameList<T>>::insert(&game_id, game_info);
			<GameCount<T>>::put(game_id);
			
			// 派發獎勵的區塊(賭注區塊挖出的後一塊開獎)
			let draw_block_number = bet_block_number + 1u32.into();
			<DrawMap<T>>::insert(&draw_block_number, game_id);

			// 進行質押
			T::Chips::reserve(&sender, amount).map_err(|_| Error::<T>::TransferError )?;

			// 通知開局紀錄
			Self::deposit_event(RawEvent::CreateGame(sender, game_id, amount, bet_block_number));
			Ok(())
		}
		
		
		#[weight = T::WeightInfo::bet()]
		pub fn bet(origin, game_id: T::GameIndex, value: ChipBalance<T>, game_mode: GameMode) -> dispatch::DispatchResult {
			// 檢核GameIndex存在
			ensure!(GameList::<T>::contains_key(game_id), Error::<T>::GameIsNotExist);
			
			// 檢查下注的遊戲，是否已經結束
			let game_info = Self::game_list(&game_id);
			let now_block_number = <frame_system::Module<T>>::block_number(); 
			ensure!(now_block_number < game_info.bet_block_number, Error::<T>::GameOver);
			
			// 檢核下注金額
			let is_over_pool = Self::check_bet_over_pool(game_id, value);
			ensure!(!is_over_pool, Error::<T>::BetAmountLimitError);

			// 檢核遊戲模式
			if game_mode != GameModeIsDouble && game_mode != GameModeIsSingle {
				return Err(Error::<T>::GameModeIsNotExist.into())
			}

			// 下注人
			let who = ensure_signed(origin)?;

			// 紀錄下注紀錄
			let new_bet_info = BetInfo{
				user: who.clone(),
				game_id: game_id,
				amount: value,
				game_mode: game_mode
			};

			// 新增下注紀錄
			let mut bet_list = BetList::<T>::get(game_id);	// 取所有下注紀錄
			let bet_index = bet_list.len();	// 下注id
			bet_list.insert(bet_index, new_bet_info);	// insert新紀錄
			<BetList<T>>::insert(&game_id, bet_list);	

			// 進行質押
			T::Chips::reserve(&who, value).map_err(|err| err)?;

			// 通知下注紀錄
			Self::deposit_event(RawEvent::Bet(who, game_id, value, game_mode, bet_index as u32));
			Ok(())
		}

		//此函數必須返回on_initialize和on_finalize消耗的權重。
		fn on_initialize(now: T::BlockNumber) -> Weight {
			T::WeightInfo::on_finalize(2u32)
		}

		fn on_finalize(now: T::BlockNumber) {
			let game_id = Self::draw_map(now);
			// 準備開獎
			if game_id.is_some() { 
				// 返回包含在其中的Some值
				let game_id = game_id.unwrap();

				// 前一筆交易的block hash
				let block_hash = <frame_system::Module<T>>::block_hash(now-1u32.into());
				let game_info = Self::game_list(&game_id);

				// 取得獲勝的模式（單 or 雙）
				let ResultGameMode = Self::get_game_result(block_hash).ok();

				// 取得下注紀錄
				let bet_list = Self::bet_list(&game_id);

				// -----------------------獎勵派發-----------------------
				// 獎池總total
				let mut owner_pool = game_info.amount;
				// owner將拿到的總金額
				let mut owner_get_total_amount = game_info.amount;

				// owner
				let owner = game_info.owner;
				for (k, v) in bet_list.iter().enumerate() {
					// 贏家
					if v.game_mode == ResultGameMode.unwrap() {	
						// 返還下注者本金
						T::Chips::unreserve(&v.user, v.amount).map_err(|err| debug::error!("err: {:?}", err));
						// owner.clone();
						// owner發放獎勵給下注者
						T::Chips::repatriate_reserved(&owner, &v.user, v.amount).map_err(|err| debug::error!("err: {:?}", err));
						
						// 通知下注者獲得金額
						Self::deposit_event(RawEvent::BettorResult(v.user.clone(), game_id, v.amount * 2u32.into(), k as u32, ResultGameMode.unwrap(), block_hash));
						
						// 計算獎池剩餘金額
						owner_pool-=v.amount;

						// Owner輸了，total get amount減少
						owner_get_total_amount-=v.amount;
					}
					// 輸家
					else{
						// 下注者發放獎勵給owner
						T::Chips::repatriate_reserved(&v.user, &owner, v.amount).map_err(|err| debug::error!("err: {:?}", err));
						// owner贏了，total get amount減少
						owner_get_total_amount+=v.amount;
					}
					
				}
				// 獎池剩餘金額返還給owner
				T::Chips::unreserve(&owner, owner_pool).map_err(|err| debug::error!("err: {:?}", err));

				// 發送通知
				Self::deposit_event(RawEvent::GameOver(owner, game_id, owner_get_total_amount, ResultGameMode.unwrap(), block_hash));
			}
		}
	}
}

impl<T: Config> Module<T> {
	// 下注後是否會超過獎池可賠金額
	fn check_bet_over_pool(game_id: T::GameIndex, bet_amount: ChipBalance<T>) -> bool{

		let game_info = Self::game_list(game_id);
		let bet_list = Self::bet_list(game_id);

		// 獎池最大可賠金額
		let pool_total = game_info.amount;

		// 總下注金額（含準下注金額）
		let mut bet_total: ChipBalance<T> = 0u32.into();
		for v in bet_list {
			bet_total += v.amount;
		}
		bet_total += bet_amount;
		
		// 返回若會超過獎池為true
		pool_total < bet_total
	}

	// 取得新的game_id
	fn next_game_id() -> sp_std::result::Result<T::GameIndex, DispatchError>{
		let game_id = Self::game_count() + 1u32.into();
		if game_id == T::GameIndex::max_value() {
			return Err(Error::<T>::GameCountOverflow.into())
		}
		Ok(game_id)
	}
	// 取得賽果
	fn get_game_result(block_hash: T::Hash) -> sp_std::result::Result<GameMode, DispatchError>{
		let block_hash_char: String = format!("{:?}", block_hash);
		let char_vec: Vec<char> = block_hash_char.chars().collect();

		let mut is_have_ans = false;
		let mut ans: u8 = 0;
		let mut n = char_vec.len() - 1;
		while !is_have_ans {
			// 字串轉型u8
			let num = char_vec[n].to_string().parse::<u8>().ok();
			if num != None {
				debug::info!("獲得答案 char_vec[n]: {:?}", char_vec[n]);
				ans = num.unwrap();
				is_have_ans = true;
			}else{
				debug::info!("跳過 char_vec[n]: {:?}", char_vec[n]);
			}
			n = n -1;
		}
		// 偶數
		if (ans % 2) == 0 {
			Ok(GameModeIsDouble)
		}
		// 奇數
		else{
			Ok(GameModeIsSingle)
		}
	}
}