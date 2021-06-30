use crate::{Error, mock::*};
use frame_support::{assert_noop, assert_ok};
use crate::RawEvent;

#[test]
fn sign_up() {
    new_test_ext().execute_with(|| {
        let user = 2;
        
        let account = "Ss01";
        let account_vec = account.clone().as_bytes().to_vec();

        let referrer_account = "staketop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();

        assert_ok!(SubGameStake::sign_up(Origin::signed(user.clone()), account_vec.clone(), referrer_account_vec.clone()));

        let want_account = account.clone().to_lowercase().as_bytes().to_vec();
        assert_eq!(want_account, SubGameStake::user_info_map(user.clone()).account);
    });
}

#[test]
fn sign_up_exists() {
    new_test_ext().execute_with(|| {
        let user = 2;
        
        let account = "Ss01";
        let account_vec = account.as_bytes().to_vec();
        
        let referrer_account = "staketop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();

        assert_ok!(SubGameStake::sign_up(Origin::signed(user.clone()), account_vec.clone(), referrer_account_vec.clone()));
        assert_noop!(SubGameStake::sign_up(Origin::signed(user.clone()), account_vec.clone(), referrer_account_vec.clone()), Error::<Test>::UserExists);

        let user = 3;
        assert_noop!(SubGameStake::sign_up(Origin::signed(user.clone()), account_vec.clone(), referrer_account_vec.clone()), Error::<Test>::UserExists);
    });
}

#[test]
fn sign_up_account_format_is_wrong() {
    new_test_ext().execute_with(|| {
        let user = 2;
        
        let account = "ABCDEFG8";
        let account_vec = account.clone().as_bytes().to_vec();

        let referrer_account = "stake01";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();

        assert_noop!(SubGameStake::sign_up(Origin::signed(user.clone()), account_vec.clone(), referrer_account_vec.clone()), Error::<Test>::AccountFormatIsWrong);
    });
}

#[test]
fn stake_user_not_exists() {
    new_test_ext().execute_with(|| {
        let user = 2;
        let amount: u64 = 1000;
        assert_noop!(SubGameStake::stake(Origin::signed(user.clone()), amount.clone()), Error::<Test>::UserNotExists);
    });
}

#[test]
fn stake() {
    new_test_ext().execute_with(|| {
        let user = 2;
        let amount: u64 = 1000;

        let account = "Ss01";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "staketop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        assert_ok!(SubGameStake::sign_up(Origin::signed(user.clone()), account_vec.clone(), referrer_account_vec.clone()));

        assert_ok!(SubGameStake::stake(Origin::signed(user.clone()), amount.clone()));
        assert_eq!(amount, Balances::reserved_balance(&user));
    });
}

#[test]
fn unlock_not_owner() {
    new_test_ext().execute_with(|| {
        let user = 2;
        let amount: u64 = 1000;
        assert_noop!(SubGameStake::unlock(Origin::signed(user.clone()), user.clone(), amount.clone()), Error::<Test>::PermissionDenied);
    });
}

#[test]
fn unlock_money_not_enough() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        let user = 2;
        let amount: u64 = 1000;

        let account = "Ss01";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "staketop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        assert_ok!(SubGameStake::sign_up(Origin::signed(user.clone()), account_vec.clone(), referrer_account_vec.clone()));

        assert_noop!(SubGameStake::unlock(Origin::signed(owner.clone()), user.clone(), amount.clone()), Error::<Test>::MoneyNotEnough);
    });
}

#[test]
fn unlock() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        let user = 2;
        let amount: u64 = 1000;

        let account = "Ss01";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "staketop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        assert_ok!(SubGameStake::sign_up(Origin::signed(user.clone()), account_vec.clone(), referrer_account_vec.clone()));

        assert_ok!(SubGameStake::stake(Origin::signed(user.clone()), amount.clone()));

        assert_ok!(SubGameStake::unlock(Origin::signed(owner.clone()), user.clone(), amount.clone()));
        assert_eq!(0, Balances::reserved_balance(&user));
    });
}

#[test]
fn withdraw() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        let user = 2;
        let amount: u64 = 1000;
        let default_balance: u64 = 1000000;

        let account = "Ss01";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "staketop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        assert_ok!(SubGameStake::sign_up(Origin::signed(user.clone()), account_vec.clone(), referrer_account_vec.clone()));

        assert_ok!(SubGameStake::withdraw(Origin::signed(owner.clone()), user.clone(), amount.clone()));
        assert_eq!(default_balance + amount, Balances::free_balance(&user));
    });
}

#[test]
fn import_stake() {
    new_test_ext().execute_with(|| {
        let owner = 1;
        let user = 2;
        let account = "ss01";
        let account_vec = account.clone().as_bytes().to_vec();
        let referrer_account = "staketop";
        let referrer_account_vec = referrer_account.as_bytes().to_vec();
        let amount: u64 = 1000;
        assert_ok!(SubGameStake::import_stake(Origin::signed(owner.clone()), user.clone(), account_vec.clone(), referrer_account_vec.clone(), amount.clone()));
        assert_eq!(amount, Balances::reserved_balance(&user));

        assert_eq!(
			System::events()[0].event,
			Event::pallet_stake(RawEvent::SignUp(user , account_vec, referrer_account_vec))
		);

        let want_account = account.clone().to_lowercase().as_bytes().to_vec();
        assert_eq!(want_account, SubGameStake::user_info_map(user.clone()).account);
    });
}