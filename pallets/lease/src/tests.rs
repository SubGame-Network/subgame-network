// Tests to be written here
use crate::mock::{new_test_ext, SubgameNFT, Lease, Origin};
use crate::mock::*;
use crate::*;
use frame_support::{assert_err, assert_ok, Hashable};

#[test]
fn set_authority() {
    new_test_ext().execute_with(|| {
        assert_ok!(SubgameNFT::mint(Origin::root(), 0, Vec::<u8>::from("test")));
        assert_ok!(Lease::add_pallet(Origin::signed(3), 1, Vec::<u8>::from("test pallet")));
        let nft_id = Vec::<u8>::from("test").blake2_256().into();
        assert_ok!(Lease::set_authority(Origin::signed(3), nft_id, 1, 0));
        let lease_info = Lease::lease_infos(nft_id);
        assert_eq!(lease_info.pallet_id, 1);
        assert_eq!(lease_info.nft_id, nft_id);
    });
}

#[test]
fn check_authority() {
    new_test_ext().execute_with(|| {
        let pallet_id=1;
        assert_ok!(SubgameNFT::mint(Origin::root(), 0, Vec::<u8>::from("test")));
        assert_ok!(Lease::add_pallet(Origin::signed(3), pallet_id, Vec::<u8>::from("test pallet")));
        
        let nft_id = Vec::<u8>::from("test").blake2_256().into();
        assert_ok!(Lease::set_authority(Origin::signed(3), nft_id, pallet_id, 0));
        let lease_info = Lease::lease_infos(nft_id);
        assert_eq!(lease_info.pallet_id, pallet_id);
        assert_eq!(lease_info.nft_id, nft_id);

        assert_ok!(Lease::check_authority(Origin::signed(0), pallet_id));
        
        assert_err!(
            Lease::check_authority(Origin::signed(3), pallet_id),
            Error::<Test>::PalletPermissionDenied
        );
    });
}

#[test]
fn add_pallet() {
    new_test_ext().execute_with(|| {
        let pallet_id=1;
        let name =  Vec::<u8>::from("test pallet");
        assert_ok!(Lease::add_pallet(Origin::signed(3), pallet_id, name.clone()));
        
        let pallets = Lease::pallets();
        
        assert_eq!(pallets.len(), 1);
        assert_eq!(pallets[0].pallet_id, pallet_id);
        assert_eq!(pallets[0].name, name);
    });
}