use crate as pallet_gamecenter;
use balances;
use frame_support::parameter_types;
use frame_system as system;
use pallet_chips;
use pallet_gametemplates_guess_hash;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

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
        Chips: pallet_chips::{Module, Call, Storage, Event<T>},
        GameGuessHashModule: pallet_gametemplates_guess_hash::{Module, Call, Storage, Event<T>},
        GameCenter: pallet_gamecenter::{Module, Call, Storage, Event<T>},
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
    pub const MasterAddress: u32 = 3;
}
impl pallet_chips::Config for Test {
    type Event = Event;
    type Balances = balances::Module<Self>;
    type ChipBalance = u128;
    type MasterAddress = MasterAddress;
    type WeightInfo = ();
}

impl pallet_gametemplates_guess_hash::Config for Test {
    type Event = Event;
    type GameIndex = u32;
    type WeightInfo = ();
    type Chips = Chips;
}

impl pallet_gamecenter::Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type GuessHash = GameGuessHashModule;
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
