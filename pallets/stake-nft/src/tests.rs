// Tests to be written here
use crate::mock::{new_test_ext, SubgameNFT, Lease, Origin, System, Timestamp};
use crate::mock::*;
use crate::*;
use frame_support::{assert_ok, assert_err,
    traits::{OnFinalize, OnInitialize},
};


/// Jump to the specified block
fn run_to_block(n: u64, t: u64) {
    while System::block_number() < n {
        SubgameNFT::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        Timestamp::set_timestamp(t);
        System::on_initialize(System::block_number());
        SubgameNFT::on_initialize(System::block_number());
    }
}


#[test]
fn stake() {
    new_test_ext().execute_with(|| {
        let program_id = 1;
        let pallet_id = 1;
        let stake_amount = 100;
        let day = 1;
        assert_ok!(SubgameStakeNft::add_program(Origin::signed(3), program_id, stake_amount, day));
        assert_ok!(Lease::add_pallet(Origin::signed(3), 1, Vec::<u8>::from("test pallet")));

        assert_ok!(SubgameStakeNft::stake(Origin::signed(4), program_id, pallet_id));

        assert_eq!(SubgameNFT::total(), 1);
        assert_eq!(SubgameNFT::total_for_account(4), 1);
        let nft_list = SubgameNFT::commodities_for_account::<u64>(4);
        let nft_id = nft_list[0].0;


        let lease_info = Lease::lease_infos(nft_id);
        assert_eq!(lease_info.pallet_id, pallet_id);
        assert_eq!(lease_info.nft_id, nft_id);
    });
}


#[test]
fn stake_expired() {
    new_test_ext().execute_with(|| {
        let program_id = 1;
        let pallet_id = 1;
        let stake_amount = 100;
        let day = 1;
        assert_ok!(SubgameStakeNft::add_program(Origin::signed(3), program_id, stake_amount, day));
        assert_ok!(Lease::add_pallet(Origin::signed(3), 1, Vec::<u8>::from("test pallet")));

        assert_ok!(SubgameStakeNft::stake(Origin::signed(4), program_id, pallet_id));
        
        assert_eq!(SubgameNFT::total(), 1);
        assert_eq!(SubgameNFT::total_for_account(4), 1);
        let nft_list = SubgameNFT::commodities_for_account::<u64>(4);
        let nft_id = nft_list[0].0;


        let lease_info = Lease::lease_infos(nft_id);
        assert_eq!(lease_info.pallet_id, pallet_id);
        assert_eq!(lease_info.nft_id, nft_id);
        let now = Timestamp::get();
        let now_ms = TryInto::<u64>::try_into(now).ok().unwrap(); // convert to u64

        // add 1 day
        let n_day = 1;
        let n_day_ms = u64::try_from(chrono::Duration::days(n_day).num_milliseconds()).ok().unwrap();
        let expires_at_ms = now_ms + n_day_ms;
        run_to_block(1, expires_at_ms);

        let nft_list = SubgameNFT::commodities_for_account::<u64>(4);
        assert_eq!(nft_list.len(), 1);
    });
}

#[test]
fn add_program() {
    new_test_ext().execute_with(|| {
        let program_id = 1;
        let stake_amount = 100;
        let day = 1;
        assert_ok!(SubgameStakeNft::add_program(Origin::signed(3), program_id, stake_amount, day));
        
        let programs = SubgameStakeNft::programs_list();
        assert_eq!(programs.len(), 1);
        assert_eq!(programs[0].program_id, 1);
        assert_eq!(programs[0].stake_amount, stake_amount);
        assert_eq!(programs[0].valid_day_count, day);

    });
}

#[test]
fn add_program_error_exist() {
    new_test_ext().execute_with(|| {
        let program_id = 1;
        let stake_amount = 100;
        let day = 1;
        assert_ok!(SubgameStakeNft::add_program(Origin::signed(3), program_id, stake_amount, day));

        assert_err!(
            SubgameStakeNft::add_program(Origin::signed(3), program_id, stake_amount, day),
            Error::<Test>::AlreadyProgram,
        );
    });
}
