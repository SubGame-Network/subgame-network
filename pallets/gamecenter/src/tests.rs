use super::*;
use crate::mock::{
    new_test_ext, Chips, Event, GameCenter, GameGuessHashModule, Origin, System, Test,
};
use crate::{mock::*, Error};
use frame_support::{
    assert_noop, assert_ok,
    traits::{OnFinalize, OnInitialize},
};

// jump to block
fn run_to_block(n: u64) {
    while System::block_number() < n {
        GameGuessHashModule::on_finalize(System::block_number());
        GameCenter::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        GameGuessHashModule::on_initialize(System::block_number());
        GameCenter::on_initialize(System::block_number());
    }
}

// 【Scenario】test create game func
#[test]
fn create_game() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange
        // A user has 100 chips
        let _ = Chips::buy_chips(Origin::signed(1), 100);

        // 【When】Act
        // A user create game
        let bet_next_few_block_num = 10u32;
        assert_ok!(GameCenter::create_game(
            Origin::signed(1),
            1,
            bet_next_few_block_num,
            100
        ));

        // 【Then】Assert

        // check current gameinstances
        let current1 = GameCenter::get_current_gameinstances(1);
        assert_eq!(current1.len(), 1);
        // check history gameinstances
        let history1 = GameCenter::get_history_gameinstances(1);
        assert_eq!(history1.len(), 0);

        let game_info = current1.last().unwrap();
        let game_instance_id = game_info.game_instance_id;
        let owner = game_info.owner;
        let bet_block_number = game_info.bet_block_number;
        let chips_pool = game_info.chips_pool;
        // check game instance id
        assert_eq!(game_instance_id, 1);
        // check owner
        assert_eq!(owner, 1);
        // check bet block num(now_block + next N block -1)
        assert_eq!(
            bet_block_number,
            System::block_number() + bet_block_number - 1
        );
        // check chips pool
        assert_eq!(chips_pool, 100);

        // check chip balance=0
        assert_eq!(Chips::chips_map(1).unwrap().balance, 0);
        // check chip reserve=100
        assert_eq!(Chips::chips_map(1).unwrap().reserve, 100);

        // jump to block
        run_to_block(40);

        // check current gameinstances
        let current2 = GameCenter::get_current_gameinstances(1);
        assert_eq!(current2.len(), 0);
        // check history gameinstances
        let history2 = GameCenter::get_history_gameinstances(1);
        assert_eq!(history2.len(), 1);
    });
}

// 【Scenario】test pay game
#[test]
fn bet() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange
        // A user has 100 chips
        let _ = Chips::buy_chips(Origin::signed(1), 100);
        // B user has 100 chips
        let _ = Chips::buy_chips(Origin::signed(2), 100);
        let bet_next_few_block_num = 10u32;
        // A user have a new game, game index = 1
        let _ = GameGuessHashModule::create_game(Origin::signed(1), bet_next_few_block_num, 100);

        // 【When】Act
        // B user下注 100 chips/ bet single
        assert_ok!(GameCenter::play_game(Origin::signed(2), 1, 100, 1));

        // 【Then】Assert
        // check chip balance=0
        assert_eq!(Chips::chips_map(2).unwrap().balance, 0);
        // check chop reserve=100
        assert_eq!(Chips::chips_map(2).unwrap().reserve, 100);

        // check bet info
        let bet_list = GameGuessHashModule::bet_list(1);
        let user = bet_list[0].user;
        let game_id = bet_list[0].game_id;
        let amount = bet_list[0].amount;
        let game_mode = bet_list[0].game_mode;
        // check player ok
        assert_eq!(user, 2);
        // check game id ok
        assert_eq!(game_id, 1);
        // check bet amount ok
        assert_eq!(amount, 100);
        // check game mode=single
        assert_eq!(game_mode, 1);
    });
}
