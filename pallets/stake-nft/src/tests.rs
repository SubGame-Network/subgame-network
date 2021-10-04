// Tests to be written here
use crate::mock::{new_test_ext, SubgameNFT, Lease, Origin};
use crate::mock::*;
use crate::*;
use frame_support::{assert_ok, Hashable};

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
        
        let nft_id = Vec::<u8>::from("test").blake2_256().into();
        assert_ok!(Lease::set_authority(Origin::signed(3), nft_id, 1, 0));

        assert_eq!(SubgameNFT::total(), 1);
        assert_eq!(SubgameNFT::total_for_account(4), 1);
        let nft_list = SubgameNFT::commodities_for_account::<u64>(4);
        let nft_id = nft_list[0].0;


        let lease_info = Lease::lease_infos(nft_id);
        assert_eq!(lease_info.pallet_id, pallet_id);
        assert_eq!(lease_info.nft_id, nft_id);
    });
}