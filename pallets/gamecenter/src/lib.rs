//! Management of Game Template instances, can create game and bet, query the specified template,  current betting games and historical games
#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, dispatch::Vec,
    weights::Weight,
};
use frame_system::ensure_signed;
use pallet_gametemplates_guess_hash::{GuessHashFunc, GuessHashTrait};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod default_weight;

#[derive(Encode, Decode, Default)]
pub struct GameInstance<GameInstanceID, Owner, DrawBlockNumber, Chips> {
    /// game instance id
    game_instance_id: GameInstanceID,
    /// create game user
    owner: Owner,
    /// bet block num (draw)
    bet_block_number: DrawBlockNumber,
    /// bet max limit
    chips_pool: Chips,
    /// check game is draw
    game_over: bool,
}

pub type GameInstanceId = u32;

pub trait WeightInfo {
    fn play_game() -> Weight;
    fn create_game() -> Weight;
    fn on_finalize(count: u32) -> Weight;
}
pub trait Config: frame_system::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type WeightInfo: WeightInfo;
    type GuessHash: GuessHashTrait + GuessHashFunc<Self::AccountId, GameInstanceId, u128>;
}

decl_storage! {
    trait Store for Module<T: Config> as GameCenterModule {
        /// List of games that can still participate in betting
        pub CurrentGameinstances get(fn get_current_gameinstances): map hasher(blake2_128_concat) u32=> Vec<GameInstance<GameInstanceId, T::AccountId, T::BlockNumber, u128>>;
        /// List of games that have been drawn
        pub HistoryGameinstances get(fn get_history_gameinstances): map hasher(blake2_128_concat) u32=> Vec<GameInstance<GameInstanceId, T::AccountId, T::BlockNumber, u128>>;
        pub PlayMap get(fn get_playmap): map hasher(blake2_128_concat) T::AccountId=> Vec<u32>;
        DrawMap get(fn draw_map): map hasher(blake2_128_concat) T::BlockNumber => Vec<GameInstanceId>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Config>::AccountId,
        BlockNumber = <T as frame_system::Config>::BlockNumber,
    {
        CreateGame(AccountId, BlockNumber),
        Bet(AccountId, u32),
    }
);

decl_error! {
    pub enum Error for Module<T: Config> {
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        #[weight = T::WeightInfo::create_game()]
        /// create template game
        pub fn create_game(origin, template_id: u32, bet_next_few_block: u32, amount: u128) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            let game_id = T::GuessHash::create_game(&sender, bet_next_few_block, amount)?;


            let block_number = <frame_system::Module<T>>::block_number();
            let _bet_block_number = block_number + bet_next_few_block.into();

            // create new game
            let gameinstances = GameInstance{
                game_instance_id: game_id,
                owner: sender,
                bet_block_number: _bet_block_number,
                chips_pool: amount,
                game_over: false,
            };
            let mut current_game_list = CurrentGameinstances::<T>::get(template_id);
            let index = current_game_list.len();
            current_game_list.insert(index, gameinstances);	// insert new record
            CurrentGameinstances::<T>::insert(&template_id, current_game_list);

            // update draw block num
            let mut drap_map = DrawMap::<T>::get(_bet_block_number);
            drap_map.insert(drap_map.len(), game_id);
            DrawMap::<T>::insert(_bet_block_number, drap_map);

            Ok(())
        }

        /// play template game
        #[weight = T::WeightInfo::play_game()]
        pub fn play_game(origin, game_id: u32, amount: u128, game_mode: u8) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            T::GuessHash::bet(&sender, game_id , amount, game_mode)?;
            Ok(())
        }
        fn on_initialize(now: T::BlockNumber) -> Weight {
            T::WeightInfo::on_finalize(2u32)
        }

        fn on_finalize(now: T::BlockNumber) {
            let game_id_list = Self::draw_map(now);
            // ready draw
            if !game_id_list.is_empty() {
                for game_id in game_id_list {
                    Self::game_over(1, game_id).ok();
                }
            }
        }
    }
}

impl<T: Config> Module<T> {
    /// The game is over, the game instance will be moved to the history
    fn game_over(template_id: u32, game_instance_id: u32) -> dispatch::DispatchResult {
        let for_current_game_list = CurrentGameinstances::<T>::get(template_id);
        let mut current_game_list = CurrentGameinstances::<T>::get(template_id);
        let mut history_game_list = HistoryGameinstances::<T>::get(template_id);
        let mut remove_game_instance_id: usize = 999999999;
        let mut tmp_data: Option<GameInstance<GameInstanceId, T::AccountId, T::BlockNumber, u128>> =
            None;
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
