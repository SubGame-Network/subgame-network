//! # stake nft Interface
//!
//! After providing users to stake SGB, they can get an nft token, and they can use special functions with nft token. SGB will be returned through redemption, and nft token will be burned at the same time. The module will provide different stake amount schemes and different valid periods. When the stake expires, nft token can no longer be used for special functions, SGB can be returned through redemption, and nft token will be burned.

use sp_std::vec::Vec;
pub trait StakeNft<AccountId> {
    type Program;
    type StakeInfo;
    fn stake_infos(account: AccountId) -> Vec<Self::StakeInfo>;
    fn programs_list() -> Vec<Self::Program>;
}
