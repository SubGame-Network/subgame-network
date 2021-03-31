use super::*;
use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop, traits::{OnFinalize, OnInitialize}};
use crate::mock::{
	Event,System,Origin, Chips, GameGuessHashModule, new_test_ext,Test,
};

// 跳到指定區塊
fn run_to_block(n: u64) {
    while System::block_number() < n {
        GameGuessHashModule::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        GameGuessHashModule::on_initialize(System::block_number());
    }
}

// 待補上測試是否正確Event
// To Do


// 【Scenario】測試開局功能
#[test]
fn create_game() {
	new_test_ext().execute_with(|| {
		// 【Given】Arrange
		// A user has 100 chips
		let _ =Chips::buy_chips(Origin::signed(1), 100);
		
		// 【When】Act
		// A user create game 
		let bet_next_few_block_num = 10u32;
		assert_ok!(GameGuessHashModule::create_game(Origin::signed(1), bet_next_few_block_num, 100) );

		// 【Then】Assert
		// 檢查籌碼餘額=0
		assert_eq!(Chips::chips_map(1).unwrap().balance, 0);
		// 檢查籌碼質押=100
		assert_eq!(Chips::chips_map(1).unwrap().reserve, 100);
		
		// 檢查遊戲資訊是否正確
		let owner = GameGuessHashModule::game_list(1).owner;
		let block_number = GameGuessHashModule::game_list(1).block_number;
		let bet_block_number = GameGuessHashModule::game_list(1).bet_block_number;
		let amount = GameGuessHashModule::game_list(1).amount;
		// 檢查開局人正確
		assert_eq!(owner, 1);	
		// 檢查賭注區塊設定正確(目前區塊+後n塊)
		assert_eq!(block_number + u64::from(bet_next_few_block_num), bet_block_number);	
		// 檢查獎池金額正確
		assert_eq!(amount , 100);	
	});
}
// 開局失敗
// To Do

// 【Scenario】測試下注功能
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
		// B user下注 100 chips/ 賭單數
		assert_ok!(GameGuessHashModule::bet(Origin::signed(2), 1, 100, 1));

		// 【Then】Assert
		// 檢查籌碼餘額=0
		assert_eq!(Chips::chips_map(2).unwrap().balance, 0);
		// 檢查籌碼質押=100
		assert_eq!(Chips::chips_map(2).unwrap().reserve, 100);

		// 檢查下注儲存參數
		let bet_list = GameGuessHashModule::bet_list(1);
		let user = bet_list[0].user;
		let game_id = bet_list[0].game_id;
		let amount = bet_list[0].amount;
		let game_mode = bet_list[0].game_mode;
		// 檢查下注人正確
		assert_eq!(user, 2);	
		// 檢查下注遊戲index正確
		assert_eq!(game_id, 1);	
		// 檢查下注金額正確
		assert_eq!(amount , 100);	
		// 檢查下注遊戲模式=單數
		assert_eq!(game_mode , 1);	
	});
}

// 下注失敗 
// To Do

// 【Scenario】測試獎勵派發是否正確
// 待解決(目前測試不到single )
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
		let _ =GameGuessHashModule::bet(Origin::signed(2), 1, 100, 1);
		// C bet single num, 100 chips
		let _ = GameGuessHashModule::bet(Origin::signed(5), 1, 100, 1);
		// D bet double num, 100 chips
		let _ = GameGuessHashModule::bet(Origin::signed(4), 1, 100, 2);

		// 【When】Act
		// 到達開獎區塊後
		run_to_block(40);

		// 【Then】Assert
		if Chips::chips_map(4).unwrap().balance == 0 {
			// when single is winner
			println!("when single is winner");
			
			// 檢查A籌碼餘額=500 - 200 + 100
			assert_eq!(Chips::chips_map(1).unwrap().balance, 400);
			// 檢查A籌碼質押=0
			assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);

			// 檢查B籌碼餘額=200
			assert_eq!(Chips::chips_map(2).unwrap().balance, 200);
			// 檢查B籌碼質押=0
			assert_eq!(Chips::chips_map(2).unwrap().reserve, 0);

			// 檢查C籌碼餘額=200
			assert_eq!(Chips::chips_map(5).unwrap().balance, 200);
			// 檢查C籌碼質押=0
			assert_eq!(Chips::chips_map(5).unwrap().reserve, 0);

			// 檢查D籌碼餘額=0
			assert_eq!(Chips::chips_map(4).unwrap().balance, 0);
			// 檢查D籌碼質押=0
			assert_eq!(Chips::chips_map(4).unwrap().reserve, 0);

		}else {
			// when double is winner
			println!("when double is winner");
			
			// 檢查A籌碼餘額=500 + 200 - 100
			assert_eq!(Chips::chips_map(1).unwrap().balance, 600);
			// 檢查A籌碼質押=0
			assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);

			// 檢查B籌碼餘額=0
			assert_eq!(Chips::chips_map(2).unwrap().balance, 0);
			// 檢查B籌碼質押=0
			assert_eq!(Chips::chips_map(2).unwrap().reserve, 0);

			// 檢查C籌碼餘額=0
			assert_eq!(Chips::chips_map(5).unwrap().balance, 0);
			// 檢查C籌碼質押=0
			assert_eq!(Chips::chips_map(5).unwrap().reserve, 0);

			// 檢查D籌碼餘額=200
			assert_eq!(Chips::chips_map(4).unwrap().balance, 200);
			// 檢查D籌碼質押=0
			assert_eq!(Chips::chips_map(4).unwrap().reserve, 0);
		}
	});
}


// To Do

// 當下注者獎勵