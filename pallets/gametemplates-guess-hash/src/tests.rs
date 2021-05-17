use crate::mock::{new_test_ext, Chips, GameGuessHashModule, Origin, System};
use frame_support::{
    assert_ok,
    traits::{OnFinalize, OnInitialize},
};

/// Jump to the specified block
fn run_to_block(n: u64) {
    while System::block_number() < n {
        GameGuessHashModule::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        GameGuessHashModule::on_initialize(System::block_number());
    }
}

/// 【Scenario】Test the deployment function
#[test]
fn create_game() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange
        // A user has 100 chips
        let _ = Chips::buy_chips(Origin::signed(1), 100);

        // 【When】Act
        // A user create game
        let bet_next_few_block_num = 10u32;
        assert_ok!(GameGuessHashModule::create_game(
            Origin::signed(1),
            bet_next_few_block_num,
            100
        ));

        // 【Then】Assert
        // Check the chip balance=0
        assert_eq!(Chips::chips_map(1).unwrap().balance, 0);
        // Check the pledge of chips=100
        assert_eq!(Chips::chips_map(1).unwrap().reserve, 100);

        // Check if the game information is correct
        let owner = GameGuessHashModule::game_list(1).owner;
        let block_number = GameGuessHashModule::game_list(1).block_number;
        let bet_block_number = GameGuessHashModule::game_list(1).bet_block_number;
        let amount = GameGuessHashModule::game_list(1).amount;
        // Check that the starter is correct
        assert_eq!(owner, 1);
        // Check that the betting block is set correctly (current block + next n blocks)
        assert_eq!(
            block_number + u64::from(bet_next_few_block_num),
            bet_block_number
        );
        // Check that the prize pool amount is correct
        assert_eq!(amount, 100);
    });
}

/// [Scenario] Test the betting function
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
        // B user bet 100 chips/ bet number
        assert_ok!(GameGuessHashModule::bet(Origin::signed(2), 1, 100, 1));

        // 【Then】Assert
        // Check the chip balance=0
        assert_eq!(Chips::chips_map(2).unwrap().balance, 0);
        // Check the pledge of chips=100
        assert_eq!(Chips::chips_map(2).unwrap().reserve, 100);

        // Check the bet storage parameters
        let bet_list = GameGuessHashModule::bet_list(1);
        let user = bet_list[0].user;
        let game_id = bet_list[0].game_id;
        let amount = bet_list[0].amount;
        let game_mode = bet_list[0].game_mode;
        // Check that the bettor is correct
        assert_eq!(user, 2);
        // Check that the betting game index is correct
        assert_eq!(game_id, 1);
        // Check that the bet amount is correct
        assert_eq!(amount, 100);
        // Check betting game mode = odd number
        assert_eq!(game_mode, 1);
    });
}

/// [Scenario] Test whether the reward distribution is correct
#[test]
fn draw() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange
        // A user has 500 chips
        let _ = Chips::buy_chips(Origin::signed(1), 500);
        // B user has 100 chips
        let _ = Chips::buy_chips(Origin::signed(2), 100);
        // C user has 100 chips
        let _ = Chips::buy_chips(Origin::signed(5), 100);
        // D user has 100 chips
        let _ = Chips::buy_chips(Origin::signed(4), 100);

        let bet_next_few_block_num = 19u32;
        // A user have a new game, game index = 1, pool = 500
        let _ = GameGuessHashModule::create_game(Origin::signed(1), bet_next_few_block_num, 500);
        // B bet single num, 100 chips
        let _ = GameGuessHashModule::bet(Origin::signed(2), 1, 100, 1);
        // C bet single num, 100 chips
        let _ = GameGuessHashModule::bet(Origin::signed(5), 1, 100, 1);
        // D bet double num, 100 chips
        let _ = GameGuessHashModule::bet(Origin::signed(4), 1, 100, 2);

        // 【When】Act
        // After reaching the lottery block
        run_to_block(40);

        // 【Then】Assert
        if Chips::chips_map(4).unwrap().balance == 0 {
            // when single is winner
            println!("when single is winner");

            // Check A chip balance=500 - 200 + 100
            assert_eq!(Chips::chips_map(1).unwrap().balance, 400);
            // Check A chip pledge=0
            assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);

            // Check B chip balance=200
            assert_eq!(Chips::chips_map(2).unwrap().balance, 200);
            // Check B chip pledge=0
            assert_eq!(Chips::chips_map(2).unwrap().reserve, 0);

            // Check C chip balance=200
            assert_eq!(Chips::chips_map(5).unwrap().balance, 200);
            // Check the pledge of C chips=0
            assert_eq!(Chips::chips_map(5).unwrap().reserve, 0);

            // Check D chip balance=0
            assert_eq!(Chips::chips_map(4).unwrap().balance, 0);
            // Check D chip pledge=0
            assert_eq!(Chips::chips_map(4).unwrap().reserve, 0);
        } else {
            // when double is winner
            println!("when double is winner");

            // Check A chip balance=500 + 200 - 100
            assert_eq!(Chips::chips_map(1).unwrap().balance, 600);
            // Check A chip pledge=0
            assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);

            // Check A chip balance=0
            assert_eq!(Chips::chips_map(2).unwrap().balance, 0);
            // Check A chip pledge=0
            assert_eq!(Chips::chips_map(2).unwrap().reserve, 0);

            // Check A chip balance=0
            assert_eq!(Chips::chips_map(5).unwrap().balance, 0);
            // Check A chip pledge=0
            assert_eq!(Chips::chips_map(5).unwrap().reserve, 0);

            // Check A chip balance=200
            assert_eq!(Chips::chips_map(4).unwrap().balance, 200);
            // Check A chip pledge=0
            assert_eq!(Chips::chips_map(4).unwrap().reserve, 0);
        }
    });
}
