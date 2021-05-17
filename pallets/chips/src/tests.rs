use crate::mock::{new_test_ext, Chips, Origin, Test};
use crate::{Error};
use frame_support::{assert_noop, assert_ok};

/// 【Scenario】Buy chips
#[test]
fn buy_chips() {
    new_test_ext().execute_with(|| {
        // First purchase of chips
        // 【Given】Arrange

        // 【When】Act
        // Buy 1000 chips
        assert_ok!(Chips::buy_chips(Origin::signed(1), 1000));
        // 【Then】Assert
        // Check the chip balance=1000
        assert_eq!(Chips::chips_map(1).unwrap().balance, 1000);
        assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);

        // Buy chips again

        // 【Given】Arrange

        // 【When】Act
        // Buy another 1,000 chips
        assert_ok!(Chips::buy_chips(Origin::signed(1), 1000));

        // 【Then】Assert
        // Check the chip balance=2000
        assert_eq!(Chips::chips_map(1).unwrap().balance, 2000);
        assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);
    });
}

/// 【Scenario】The balance is not enough to buy chips
#[test]
fn buy_chips_failed_when_not_enough_money() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange
        // F user No token

        // 【When】Act
        // 【Then】Assert
        // Buy chips && return Error MoneyNotEnough
        assert_noop!(
            Chips::buy_chips(Origin::signed(6), 1000),
            Error::<Test>::MoneyNotEnough
        );
    });
}

/// 【Scenario】Redemption
#[test]
fn redemption_all_chips() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange
        // A user has 1000 chips
        let _ = Chips::buy_chips(Origin::signed(1), 1000);

        // 【When】Act
        // Redemption
        assert_ok!(Chips::redemption(Origin::signed(1), 1000));
        // 【Then】Assert
        // Check the chip balance=0
        assert_eq!(Chips::chips_map(1).unwrap().balance, 0);
        assert_eq!(Chips::chips_map(1).unwrap().reserve, 0);
    });
}
/// 【Scenario】Redemption failed (no chips have been purchased)
#[test]
fn redemption_failded_when_never_bought_chips() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange
        // F user haven't bought any chips

        // 【When】Act
        // 【Then】Assert
        // Redeem && Return Error ChipsIsNotExist
        assert_noop!(
            Chips::redemption(Origin::signed(1), 1000),
            Error::<Test>::NeverBoughtChips
        );
    });
}
/// 【Scenario】Redemption failed (insufficient chips)
#[test]
fn redemption_failded_when_chips_is_not_enough() {
    new_test_ext().execute_with(|| {
        // 【Given】Arrange
        // F user has 10 chips
        let _ = Chips::buy_chips(Origin::signed(1), 10);

        // 【When】Act
        // 【Then】Assert
        // Redeem && Return Error ChipsIsNotExist
        assert_noop!(
            Chips::redemption(Origin::signed(1), 1000),
            Error::<Test>::ChipsIsNotEnough
        );
        assert_eq!(Chips::chips_map(1).unwrap().balance, 10);
    });
}
