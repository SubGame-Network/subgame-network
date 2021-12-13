use crate as pallet_tspwhitelist;
use balances;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use frame_system::{EnsureRoot};
use pallet_subgame_assets;
use pallet_timestamp;

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
        SubGameAssets: pallet_subgame_assets::{Module, Call, Storage, Event<T>},
        PalletTimestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        TSPWhitelist: pallet_tspwhitelist::{Module, Call, Storage, Event<T>},
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
    pub const OwnerAddress: u64 = 1;
}
impl pallet_tspwhitelist::Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type Currency = balances::Module<Self>;
    type OwnerAddress = OwnerAddress;
}

pub const MILLICENTS: u64 = 10_000_000_000;
parameter_types! {
    pub const AssetDepositBase: u64 = 100 * MILLICENTS;
    pub const AssetDepositPerZombie: u64 = 1 * MILLICENTS;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: u64 = 10 * MILLICENTS;
    pub const MetadataDepositPerByte: u64 = 1 * MILLICENTS;
}
impl pallet_subgame_assets::Config for Test {
    type Event = Event;
    type SGAssetBalance = u64;
    type AssetId = u32;
    type Currency = balances::Module<Self>;
    type ForceOrigin = EnsureRoot<Self::AccountId>;
    type AssetDepositBase = AssetDepositBase;
    type AssetDepositPerZombie = AssetDepositPerZombie;
    type StringLimit = StringLimit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type WeightInfo = ();
}

impl pallet_timestamp::Config for Test {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ();
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
            (12500082579580134024, 10000000000000000),
            (1, 10000000000000000),
            (2, 10000000000000000),
            (3, 10000000000000000),
            (4, 10000000000000000),
            (5, 10000000000000000),
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext: sp_io::TestExternalities = t.into();
    ext.execute_with(|| System::set_block_number(1));
    ext
}
