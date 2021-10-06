use crate::mock::{new_test_ext, SubgameStakeNft, Lease, Origin};
use crate::mock::*;
use crate::*;
use frame_support::{
    assert_ok, assert_err,
};

// 【Scenario】test create game func
#[test]
fn demo() {
    new_test_ext().execute_with(|| {
        let program_id = 1;
        let pallet_id = 1;
        let stake_amount = 100;
        let day = 1;
        assert_ok!(SubgameStakeNft::add_program(Origin::signed(3), program_id, stake_amount, day));
        assert_ok!(Lease::add_pallet(Origin::signed(3), 1, Vec::<u8>::from("test pallet")));

        assert_ok!(SubgameStakeNft::stake(Origin::signed(4), program_id, pallet_id));

        
        assert_eq!(
            DemoGame::call_success(4),
            0
        );
        assert_ok!(
            DemoGame::demo(Origin::signed(4))
        );
        assert_eq!(
            DemoGame::call_success(4),
            1
        );
        
    });




}
#[test]
fn demo_error_permission() {
    new_test_ext().execute_with(|| {
        assert_ok!(Lease::add_pallet(Origin::signed(3), 1, Vec::<u8>::from("test pallet")));
        assert_err!(
            DemoGame::demo(Origin::signed(10)),
            Error::<Test>::PermissionDenied
        );
    });
}
