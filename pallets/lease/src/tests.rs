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

// #[test]
// fn mint_err_non_admin() {
//     new_test_ext().execute_with(|| {
//         assert_err!(
//             SUT::mint(Origin::signed(1), 1, Vec::<u8>::default()),
//             sp_runtime::DispatchError::BadOrigin
//         );
//     });
// }

// #[test]
// fn mint_err_dupe() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(SUT::mint(Origin::root(), 1, Vec::<u8>::default()));

//         assert_err!(
//             SUT::mint(Origin::root(), 2, Vec::<u8>::default()),
//             Error::<Test, DefaultInstance>::CommodityExists
//         );
//     });
// }

// #[test]
// fn mint_err_max_user() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(SUT::mint(Origin::root(), 1, vec![]));
//         assert_ok!(SUT::mint(Origin::root(), 1, vec![0]));

//         assert_err!(
//             SUT::mint(Origin::root(), 1, vec![1]),
//             Error::<Test, DefaultInstance>::TooManyCommoditiesForAccount
//         );
//     });
// }

// #[test]
// fn mint_err_max() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(SUT::mint(Origin::root(), 1, vec![]));
//         assert_ok!(SUT::mint(Origin::root(), 2, vec![0]));
//         assert_ok!(SUT::mint(Origin::root(), 3, vec![1]));
//         assert_ok!(SUT::mint(Origin::root(), 4, vec![2]));
//         assert_ok!(SUT::mint(Origin::root(), 5, vec![3]));

//         assert_err!(
//             SUT::mint(Origin::root(), 6, vec![4]),
//             Error::<Test, DefaultInstance>::TooManyCommodities
//         );
//     });
// }

// #[test]
// fn burn() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(SUT::mint(Origin::root(), 1, Vec::<u8>::from("test")));
//         assert_eq!(SUT::total_for_account(1), 1);

//         let assets = SUT::assets_for_account(&(1 as u64));

//         assert_ok!(SUT::burn(Origin::signed(1), assets[0].0));

//         assert_eq!(SUT::total(), 0);
//         assert_eq!(SUT::burned(), 1);
//         assert_eq!(SUT::total_for_account(1), 0);
//         assert_eq!(SUT::commodities_for_account::<u64>(1), vec![]);
//         assert_eq!(
//             SUT::account_for_commodity::<H256>(Vec::<u8>::default().blake2_256().into()),
//             0
//         );
//     });
// }

// #[test]
// fn burn_err_not_owner() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(SUT::mint(Origin::root(), 1, Vec::<u8>::default()));

//         assert_err!(
//             SUT::burn(Origin::signed(2), Vec::<u8>::default().blake2_256().into()),
//             Error::<Test, DefaultInstance>::NotCommodityOwner
//         );
//     });
// }

// #[test]
// fn burn_err_not_exist() {
//     new_test_ext().execute_with(|| {
//         assert_err!(
//             SUT::burn(Origin::signed(1), Vec::<u8>::default().blake2_256().into()),
//             Error::<Test, DefaultInstance>::NotCommodityOwner
//         );
//     });
// }

// #[test]
// fn transfer() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(SUT::mint(Origin::root(), 1, "test".into()));

//         let assets = SUT::assets_for_account(&(1 as u64));

//         assert_ok!(SUT::transfer(Origin::signed(1), 2, assets[0].0));

//         assert_eq!(SUT::total(), 1);
//         assert_eq!(SUT::burned(), 0);
//         assert_eq!(SUT::total_for_account(1), 0);
//         assert_eq!(SUT::total_for_account(2), 1);
//         assert_eq!(SUT::commodities_for_account::<u64>(1), vec![]);
//         let commodities_for_account = SUT::commodities_for_account::<u64>(2);
//         assert_eq!(commodities_for_account.len(), 1);
//         assert_eq!(commodities_for_account[0].0, assets[0].0);
//         assert_eq!(commodities_for_account[0].1, Vec::<u8>::from("test"));
//         assert_eq!(
//             SUT::account_for_commodity::<H256>(commodities_for_account[0].0),
//             2
//         );
//     });
// }

// #[test]
// fn transfer_err_not_owner() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(SUT::mint(Origin::root(), 1, Vec::<u8>::default()));

//         assert_err!(
//             SUT::transfer(
//                 Origin::signed(0),
//                 2,
//                 Vec::<u8>::default().blake2_256().into()
//             ),
//             Error::<Test, DefaultInstance>::NotCommodityOwner
//         );
//     });
// }

// #[test]
// fn transfer_err_not_exist() {
//     new_test_ext().execute_with(|| {
//         assert_err!(
//             SUT::transfer(
//                 Origin::signed(1),
//                 2,
//                 Vec::<u8>::default().blake2_256().into()
//             ),
//             Error::<Test, DefaultInstance>::NotCommodityOwner
//         );
//     });
// }

// #[test]
// fn transfer_err_max_user() {
//     new_test_ext().execute_with(|| {
//         assert_ok!(SUT::mint(Origin::root(), 1, vec![0]));
//         assert_ok!(SUT::mint(Origin::root(), 1, vec![1]));
//         assert_ok!(SUT::mint(Origin::root(), 2, Vec::<u8>::default()));
//         assert_eq!(
//             SUT::account_for_commodity::<H256>(Vec::<u8>::default().blake2_256().into()),
//             2
//         );

//         assert_err!(
//             SUT::transfer(
//                 Origin::signed(2),
//                 1,
//                 Vec::<u8>::default().blake2_256().into()
//             ),
//             Error::<Test, DefaultInstance>::TooManyCommoditiesForAccount
//         );
//     });
// }