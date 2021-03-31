use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use crate::mock::{
	Event,System,Origin,Chips, new_test_ext,Test
};

// 待補上測試是否正確Event
// To Do


// 【Scenario】購買籌碼
#[test]
fn buy_chips() {
	new_test_ext().execute_with(|| {
		// 首次購買籌碼
		// 【Given】Arrange

		// 【When】Act
		// 購買1000籌碼
		assert_ok!(Chips::buy_chips(Origin::signed(1), 1000));
		// 【Then】Assert
		// 檢查籌碼餘額=1000
		assert_eq!(Chips::chips_map(1).unwrap().balance, 1000);
		assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);

		// 再次購買籌碼

		// 【Given】Arrange
		
		// 【When】Act
		// 再購買1000籌碼
		assert_ok!(Chips::buy_chips(Origin::signed(1), 1000));
		
		// 【Then】Assert
		// 檢查籌碼餘額=2000
		assert_eq!(Chips::chips_map(1).unwrap().balance, 2000);
		assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);
	});
}

// 【Scenario】餘額不足以購買籌碼
#[test]
fn buy_chips_failed_when_not_enough_money() {
	new_test_ext().execute_with(|| {
		// 【Given】Arrange
		// F user 沒有token

		// 【When】Act
		// 【Then】Assert
		// 購買籌碼 && 返回Error MoneyNotEnough
		assert_noop!(Chips::buy_chips(Origin::signed(6), 1000), Error::<Test>::MoneyNotEnough);
	});
}

// 【Scenario】贖回
#[test]
fn redemption_all_chips() {
	new_test_ext().execute_with(|| {

		// 【Given】Arrange
		// A user has 1000 chips
		let _ = Chips::buy_chips(Origin::signed(1), 1000);

		// 【When】Act
		// 贖回
		assert_ok!(Chips::redemption(Origin::signed(1), 1000));
		// 【Then】Assert
		// 檢查籌碼餘額=0
		assert_eq!(Chips::chips_map(1).unwrap().balance, 0);
		assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);
	});
}
// 【Scenario】贖回失敗（沒購買過籌碼）
#[test]
fn redemption_failded_when_never_bought_chips() {
	new_test_ext().execute_with(|| {
		// 【Given】Arrange
		// F user 沒有買過籌碼

		// 【When】Act
		// 【Then】Assert
		// 贖回 && 返回Error ChipsIsNotExist
		assert_noop!(Chips::redemption(Origin::signed(1), 1000), Error::<Test>::NeverBoughtChips);
	});
}
// 【Scenario】贖回失敗（籌碼不足）
#[test]
fn redemption_failded_when_chips_is_not_enough() {
	new_test_ext().execute_with(|| {
		// 【Given】Arrange
		// F user has 10 chips
		let _ = Chips::buy_chips(Origin::signed(1), 10);

		// 【When】Act
		// 【Then】Assert
		// 贖回 && 返回Error ChipsIsNotExist
		assert_noop!(Chips::redemption(Origin::signed(1), 1000), Error::<Test>::ChipsIsNotEnough);
		assert_eq!(Chips::chips_map(1).unwrap().balance, 10);
	});
}


// Moudle Test
// To Do

// 質押


// 解押


// 轉移
