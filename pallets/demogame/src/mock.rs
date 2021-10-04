use crate as pallet_demogame;
use balances;
use frame_support::parameter_types;
use frame_system as system;
use pallet_stake_nft;
use pallet_lease;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

pub const MILLISECS_PER_BLOCK: u64 = 12000;
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Balances: balances::{Module, Call, Storage, Config<T>, Event<T>},
        SubgameNFT: pallet_nft::{Module, Call, Storage, Event<T>},
        SubgameStakeNft: pallet_stake_nft::{Module, Call, Storage, Event<T>},
        Lease: pallet_lease::{Module, Call, Storage, Event<T>},
        DemoGame: pallet_demogame::{Module, Call, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type AccountData = balances::AccountData<u64>;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 500;
    pub const MaxLocks: u32 = 50;
}
impl balances::Config for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
    pub const ModuleOwner: u32 = 3;
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
// #[derive(Clone, Eq, PartialEq)]
// pub struct Test;

parameter_types! {
    pub const CommodityLimit: u128 =    1000000000000000000000;
    pub const UserCommodityLimit: u64 = 10000000000000000000;
}
impl pallet_nft::Config for Test {
    type CommodityAdmin = frame_system::EnsureRoot<Self::AccountId>;
    type CommodityLimit = CommodityLimit;
    type UserCommodityLimit = UserCommodityLimit;
    type Event = Event;
}

impl pallet_lease::Config for Test {
    type Event = Event;
    type PalletId = u64;
    type UniqueAssets = SubgameNFT;
    type OwnerAddress = ModuleOwner;
}

impl pallet_stake_nft::Config for Test {
    type ProgramId = u64;
    type PalletId = u64;
    type Balances = Balances;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type OwnerAddress = ModuleOwner;
    type Event = Event;
}

parameter_types! {
    pub const PalletIdPalletDemogame: u64 =    1;
}

impl pallet_demogame::Config for Test {
    type PalletId = PalletIdPalletDemogame;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type Event = Event;
}


parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Test {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}


/// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    balances::GenesisConfig::<Test> {
        // Provide some initial balances
        balances: vec![
            (1, 1000000),
            (2, 1000000),
            (3, 1000000),
            (4, 1000000),
            (5, 1000000),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();
    let mut ext: sp_io::TestExternalities = t.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
