#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use pallet_grandpa::fg_primitives;
use pallet_grandpa::{AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList};
use sp_api::impl_runtime_apis;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::traits::{
    AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, OpaqueKeys, NumberFor, Verify,
};
use sp_runtime::{
    create_runtime_str, generic, impl_opaque_keys,
    transaction_validity::{TransactionSource, TransactionValidity, TransactionPriority},
    curve::PiecewiseLinear,
    ApplyExtrinsicResult, MultiSignature,
    ModuleId,
};
use frame_system::{EnsureRoot, EnsureOneOf, limits::{BlockWeights, BlockLength}};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

// A few exports that help ease life for downstream crates.
pub use frame_support::{
    construct_runtime, ord_parameter_types, parameter_types,
    traits::{KeyOwnerProofSystem, Randomness, LockIdentifier},
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        IdentityFee, Weight, DispatchClass,
    },
    StorageValue,
};
pub use pallet_balances::Call as BalancesCall;
pub use pallet_timestamp::Call as TimestampCall;
use sp_core::u32_trait::{_1, _2, _3, _5};
pub use pallet_staking::StakerStatus;
use pallet_authorship;
use pallet_im_online;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use static_assertions::const_assert;
use pallet_transaction_payment::CurrencyAdapter;
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use sp_runtime::{Percent, Perbill, Permill, RuntimeDebug};

/*** Pallet Contracts ***/
use pallet_contracts::weights::WeightInfo;
/*** Pallet Contracts ***/
use pallet_session::historical as session_historical;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;


pub type Amount = i128;

// chips
pub use pallet_chips;
// game template
pub use pallet_gametemplates;
// game center
pub use pallet_gamecenter;
// game 1：guess hash
pub use pallet_gametemplates_guess_hash;
// bridge
pub use pallet_bridge;
// stake
pub use pallet_stake;
// swap
pub use pallet_swap;
// TSP Whitelist
pub use pallet_tspwhitelist;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
    use super::*;

    pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

    /// Opaque block header type.
    pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;

    impl_opaque_keys! {
        pub struct SessionKeys {
            pub babe: Babe,
            pub grandpa: Grandpa,
			pub im_online: ImOnline,
			pub authority_discovery: AuthorityDiscovery,
        }
    }
}

type MoreThanHalfCouncil = EnsureOneOf<
	AccountId,
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, CouncilCollective>
>;

type ApproveOrigin = EnsureOneOf<
	AccountId,
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<_3, _5, AccountId, CouncilCollective>
>;
pub const EPOCH_DURATION_IN_BLOCKS: BlockNumber = HOURS;

pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: create_runtime_str!("subgame"),
    impl_name: create_runtime_str!("subgame"),
    authoring_version: 1,
    spec_version: 176,
    impl_version: 1,
    apis: RUNTIME_API_VERSIONS,
    transaction_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 6000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
/// We allow for 2 seconds of compute with a 6 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = 2 * WEIGHT_PER_SECOND;

parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
    pub const SS58Prefix: u8 = 27;
}

// Configure FRAME pallets to include in runtime.

impl frame_system::Config for Runtime {
    /// The basic call filter to use in dispatchable.
    type BaseCallFilter = ();
    /// Block & extrinsics weights: base values and limits.
    type BlockWeights = RuntimeBlockWeights;
    /// The maximum length of a block (in bytes).
    type BlockLength = RuntimeBlockLength;
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The aggregated dispatch type that is available for extrinsics.
    type Call = Call;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = AccountIdLookup<AccountId, ()>;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Index;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous origin type.
    type Origin = Origin;
    /// Maximum number of block number to block hash mappings to keep (oldest pruned first).
    type BlockHashCount = BlockHashCount;
    /// The weight of database operations that the runtime can invoke.
    type DbWeight = RocksDbWeight;
    /// Version of the runtime.
    type Version = Version;
    /// Converts a module to the index of the module in `construct_runtime!`.
    ///
    /// This type is being generated by `construct_runtime!`.
    type PalletInfo = PalletInfo;
    /// What to do if a new account is created.
    type OnNewAccount = ();
    /// What to do if an account is fully reaped from the system.
    type OnKilledAccount = ();
    /// The data to be stored in an account.
    type AccountData = pallet_balances::AccountData<Balance>;
    /// Weight information for the extrinsics of this pallet.
    type SystemWeightInfo = ();
    /// This is used as an identifier of the chain. 42 is the generic substrate prefix.
    type SS58Prefix = SS58Prefix;
}

impl pallet_grandpa::Config for Runtime {
    type Event = Event;
    type Call = Call;

    type KeyOwnerProofSystem = ();

    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;

    type HandleEquivocation = ();

    type WeightInfo = ();
}


/// Wasm binary unwrapped. If built with `SKIP_WASM_BUILD`, the function panics.
#[cfg(feature = "std")]
pub fn wasm_binary_unwrap() -> &'static [u8] {
	WASM_BINARY.expect("Development wasm binary is not available. This means the client is \
						built with `SKIP_WASM_BUILD` flag and it is only usable for \
						production chains. Please rebuild with the flag disabled.")
}

impl pallet_authority_discovery::Config for Runtime {}

pub type Moment = u64;
parameter_types! {
    pub const EpochDuration: u64 = EPOCH_DURATION_IN_BLOCKS as u64;
    pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
    pub const ReportLongevity: u64 = 
        BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get(); // Kusama
}

impl pallet_babe::Config for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;
    type KeyOwnerProofSystem = Historical; // Historical;
    type KeyOwnerProof =
    <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, pallet_babe::AuthorityId)>>::Proof;
    type KeyOwnerIdentification =
    <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, pallet_babe::AuthorityId)>>::IdentificationTuple;
    type HandleEquivocation = pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
    type WeightInfo = ();
}

/*** Pallet authorship ***/
parameter_types! {
	pub const UncleGenerations: BlockNumber = 5;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (Staking, ImOnline);
}
/*** Pallet authorship ***/

/*** Pallet offences ***/
parameter_types! {
    pub OffencesWeightSoftLimit: Weight = Perbill::from_percent(60) *
        RuntimeBlockWeights::get().max_block;
}

impl pallet_offences::Config for Runtime {
	type Event = Event;
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
	type WeightSoftLimit = OffencesWeightSoftLimit;
}
/*** Pallet offences ***/

#[allow(non_fmt_panic)]
pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	// pub const SessionsPerEra: sp_staking::SessionIndex = 24; // 24 hours
    pub const SessionsPerEra: sp_staking::SessionIndex = 1; // 1 hours
	pub const BondingDuration: pallet_staking::EraIndex = 7; // 1 * 7 hours
	pub const SlashDeferDuration: pallet_staking::EraIndex = 1; // 24 hours
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const ElectionLookahead: BlockNumber = EPOCH_DURATION_IN_BLOCKS / 4;

	pub const MaxNominatorRewardedPerValidator: u32 = 256;
	pub const MaxIterations: u32 = 5;
	// 0.05%. The higher the value, the more strict solution acceptance becomes.
	pub MinSolutionScoreBump: Perbill = Perbill::from_rational_approximation(5u32, 10_000);
	pub StakingUnsignedPriority: TransactionPriority =
		Perbill::from_percent(90) * TransactionPriority::max_value();
}

parameter_types! {
	pub OffchainSolutionWeightLimit: Weight = RuntimeBlockWeights::get()
		.get(DispatchClass::Normal)
		.max_extrinsic
		.expect("Normal extrinsics have weight limit configured by default; qed")
		.saturating_sub(BlockExecutionWeight::get());
}

pub type CurrencyToVote = frame_support::traits::U128CurrencyToVote;

impl pallet_staking::Config for Runtime {
	type Call = Call;
	type Event = Event;
	type UnixTime = Timestamp;

	type Currency = Balances;
	type CurrencyToVote = CurrencyToVote;
	type RewardRemainder = (); // TODO: Treasury
	type Slash = Treasury; // TODO: Treasury
	type Reward = (); // rewards are minted from the voi

	type SessionInterface = Self;
	type NextNewSession = Session;
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	/// A super-majority of the council can cancel the slash.
	type SlashCancelOrigin = EnsureRoot<Self::AccountId>; // TODO: SlashCancelOrigin
	type RewardCurve = RewardCurve;
	type ElectionLookahead = ElectionLookahead;

	type MinSolutionScoreBump = MinSolutionScoreBump;
	type MaxIterations = MaxIterations;
	type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
	type UnsignedPriority = StakingUnsignedPriority; // Kusama
	type OffchainSolutionWeightLimit = OffchainSolutionWeightLimit;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const LaunchPeriod: BlockNumber = 7 * MINUTES;
	pub const VotingPeriod: BlockNumber = 7 * MINUTES;
	pub const FastTrackVotingPeriod: BlockNumber = 3 * 1_000 / (MILLISECS_PER_BLOCK as BlockNumber);
	pub const MinimumDeposit: Balance = 100 * CENTS;
	pub const EnactmentPeriod: BlockNumber = 8 * MINUTES;
	pub const CooloffPeriod: BlockNumber = 7 * MINUTES;
	// One cent: $10,000 / MB
	pub const PreimageByteDeposit: Balance = 10 * MILLICENTS;
	pub const InstantAllowed: bool = true;
	pub const MaxVotes: u32 = 100;
	pub const MaxProposals: u32 = 100;
}

// impl pallet_democracy::Config for Runtime {
// 	type Proposal = Call;
// 	type Event = Event;
// 	type Currency = Balances;
// 	type EnactmentPeriod = EnactmentPeriod;
// 	type LaunchPeriod = LaunchPeriod;
// 	type VotingPeriod = VotingPeriod;
// 	type MinimumDeposit = MinimumDeposit;
// 	/// A straight majority of the council can decide what their next motion is.
// 	type ExternalOrigin = pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilCollective>;
// 	/// A majority can have the next scheduled referendum be a straight majority-carries vote.
// 	type ExternalMajorityOrigin = pallet_collective::EnsureProportionAtLeast<_1, _2, AccountId, CouncilCollective>;
// 	/// A unanimous council can have the next scheduled referendum be a straight default-carries
// 	/// (NTB) vote.
// 	type ExternalDefaultOrigin = pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, CouncilCollective>;
// 	/// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
// 	/// be tabled immediately and with a shorter voting/enactment period.
// 	type FastTrackOrigin = pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, TechnicalCollective>;
// 	type InstantOrigin = pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, TechnicalCollective>;
// 	type InstantAllowed = InstantAllowed;
// 	type FastTrackVotingPeriod = FastTrackVotingPeriod;
// 	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
// 	type CancellationOrigin = EnsureOneOf<
// 		AccountId,
// 		EnsureRoot<AccountId>,
// 		pallet_collective::EnsureProportionAtLeast<_2, _3, AccountId, CouncilCollective>,
// 	>;
// 	type BlacklistOrigin = EnsureRoot<AccountId>;
// 	// To cancel a proposal before it has been passed, the technical committee must be unanimous or
// 	// Root must agree.
// 	type CancelProposalOrigin = EnsureOneOf<
// 		AccountId,
// 		EnsureRoot<AccountId>,
// 		pallet_collective::EnsureProportionAtLeast<_1, _1, AccountId, TechnicalCollective>,
// 	>;
// 	// Any single technical committee member may veto a coming council proposal, however they can
// 	// only do it once and it lasts only for the cooloff period.
// 	type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCollective>;
// 	type CooloffPeriod = CooloffPeriod;
// 	type PreimageByteDeposit = PreimageByteDeposit;
// 	type OperationalPreimageOrigin = pallet_collective::EnsureMember<AccountId, CouncilCollective>;
// 	type Slash = Treasury;
// 	type Scheduler = Scheduler;
// 	type PalletsOrigin = OriginCaller;
// 	type MaxVotes = MaxVotes;
// 	type WeightInfo = pallet_democracy::weights::SubstrateWeight<Runtime>;
// 	type MaxProposals = MaxProposals;
// }

parameter_types! {
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
}

impl pallet_session::Config for Runtime {
	type Event = Event;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type ShouldEndSession = Babe;
	type NextSessionRotation = Babe;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type SessionHandler = <opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = opaque::SessionKeys;
	type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}


pub mod constants;
use constants::{time::*};
parameter_types! {
    pub const SessionDuration: BlockNumber = EPOCH_DURATION_IN_SLOTS as _;
    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
}

impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type Event = Event;
	type ValidatorSet = Historical;
	type SessionDuration = SessionDuration;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
	pub const CandidacyBond: Balance = 1 * DOLLARS;
	// 1 storage item created, key size is 32 bytes, value size is 16+16.
	pub const VotingBondBase: Balance = deposit(1, 64);
	// additional data per vote is 32 bytes (account id).
	pub const VotingBondFactor: Balance = deposit(0, 32);
	/// Daily council elections
	pub const TermDuration: BlockNumber = 24 * HOURS;
	pub const DesiredMembers: u32 = 19;
	pub const DesiredRunnersUp: u32 = 19;

	// pub const CandidacyBond: Balance = 10 * DOLLARS;
	// pub const VotingBond: Balance = DOLLARS;
	// pub const TermDuration: BlockNumber = 7 * DAYS;
	// pub const DesiredMembers: u32 = 13;
	// pub const DesiredRunnersUp: u32 = 7;
	pub const ElectionsPhragmenModuleId: LockIdentifier = *b"phrelect";
}

// Make sure that there are no more than MaxMembers members elected via phragmen.
const_assert!(DesiredMembers::get() <= CouncilMaxMembers::get());

impl pallet_elections_phragmen::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type ChangeMembers = Council;
	type InitializeMembers = Council;

	type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
	type CandidacyBond = CandidacyBond;
	type VotingBondBase = VotingBondBase;
	type VotingBondFactor = VotingBondFactor;

	type LoserCandidate = Treasury;
	type KickedMember = Treasury;
	type DesiredMembers = DesiredMembers;
	type DesiredRunnersUp = DesiredRunnersUp;
	type TermDuration = TermDuration;

	type ModuleId = ElectionsPhragmenModuleId;
	type WeightInfo = ();
}

parameter_types! {
    pub const TreasuryModuleId: ModuleId = ModuleId(*b"py/trsry");
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = DOLLARS;
	pub const SpendPeriod: BlockNumber = DAYS;
	pub const Burn: Permill = Permill::from_percent(0);

	pub const BountyDepositBase: Balance = DOLLARS;
	pub const BountyDepositPayoutDelay: BlockNumber = DAYS;
	pub const BountyUpdatePeriod: BlockNumber = 14 * DAYS;
	pub const BountyCuratorDeposit: Permill = Permill::from_percent(50);
	pub const BountyValueMinimum: Balance = 5 * DOLLARS;

	pub const TipCountdown: BlockNumber = DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(10);
	pub const TipReportDepositBase: Balance = DOLLARS;
	pub const DataDepositPerByte: Balance = CENTS;
	pub const MaximumReasonLength: u32 = 16384;
}

impl pallet_treasury::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type ModuleId = TreasuryModuleId;
	type ApproveOrigin = ApproveOrigin;
	type RejectOrigin = MoreThanHalfCouncil;


	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type SpendFunds = (); // Bounties;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type OnSlash = Treasury;
	type BurnDestination = ();
	type WeightInfo = ();
}

impl pallet_bounties::Config for Runtime {
	type Event = Event;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type BountyCuratorDeposit = BountyCuratorDeposit;
	type BountyValueMinimum = BountyValueMinimum;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type WeightInfo = ();

}
impl pallet_tips::Config for Runtime {
	type Event = Event;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type Tippers = ElectionsPhragmen; // Kusama
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type WeightInfo = ();
}
parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 3 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}

type CouncilCollective = pallet_collective::Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;

	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
}

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = 3 * DAYS;
	pub const TechnicalMaxProposals: u32 = 100;
	pub const TechnicalMaxMembers: u32 = 100;
}

type TechnicalCollective = pallet_collective::Instance2;
impl pallet_collective::Config<TechnicalCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = TechnicalMotionDuration;
	type MaxProposals = TechnicalMaxProposals;
	type MaxMembers = TechnicalMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
}

impl pallet_membership::Config<pallet_membership::Instance1> for Runtime {
	type Event = Event;
	type AddOrigin = MoreThanHalfCouncil;
	type RemoveOrigin = MoreThanHalfCouncil;
	type SwapOrigin = MoreThanHalfCouncil;
	type ResetOrigin = MoreThanHalfCouncil;
	type PrimeOrigin = MoreThanHalfCouncil;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
}

parameter_types! {
	pub const IndexDeposit: Balance = DOLLARS;
}

impl pallet_indices::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type Deposit = IndexDeposit;
	type AccountIndex = AccountIndex;
	type WeightInfo = ();
}
parameter_types! {
	pub const MultisigDepositBase: Balance = 500 * MILLICENTS;
	pub const MultisigDepositFactor: Balance = 100 * MILLICENTS;
	pub const MaxSignatories: u16 = 100;
}

impl pallet_multisig::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type DepositBase = MultisigDepositBase;
	type DepositFactor = MultisigDepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = ();
}

parameter_types! {
	pub const ConfigDepositBase: Balance = 10 * CENTS;
	pub const FriendDepositFactor: Balance = CENTS;
	pub const MaxFriends: u16 = 9;
	pub const RecoveryDeposit: Balance = 10 * CENTS;
}

impl pallet_recovery::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ConfigDepositBase = ConfigDepositBase;
	type FriendDepositFactor = FriendDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
}

parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub const ProxyDepositBase: Balance = deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = deposit(0, 33);
	pub const MaxProxies: u16 = 32;
	pub const AnnouncementDepositBase: Balance = deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
	pub const MaxPending: u16 = 32;
}

impl pallet_proxy::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ProxyType = ();
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = MaxProxies;
	type WeightInfo = ();
	type MaxPending = MaxPending;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}

impl pallet_utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type WeightInfo = ();
}
parameter_types! {
    pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Babe;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

parameter_types! {
    pub const TransactionByteFee: Balance = 1;
}

impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ();
}

impl pallet_sudo::Config for Runtime {
    type Event = Event;
    type Call = Call;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
	where
		Call: From<C>,
{
	type OverarchingCall = Call;
	type Extrinsic = UncheckedExtrinsic;
}


/*** Pallet Contracts ***/
pub const MILLICENTS: Balance = 1_000_000_000;
pub const CENTS: Balance = 1_000 * MILLICENTS;
pub const DOLLARS: Balance = 100 * CENTS;

const fn deposit(items: u32, bytes: u32) -> Balance {
    items as Balance * 15 * CENTS + (bytes as Balance) * 6 * CENTS
}

/// We assume that ~10% of the block weight is consumed by `on_initalize` handlers.
/// This is used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(10);

parameter_types! {
    pub const TombstoneDeposit: Balance = deposit(
        1,
        sp_std::mem::size_of::<pallet_contracts::ContractInfo<Runtime>>() as u32
    );
    pub const DepositPerContract: Balance = TombstoneDeposit::get();
    pub const DepositPerStorageByte: Balance = deposit(0, 1);
    pub const DepositPerStorageItem: Balance = deposit(1, 0);
    pub RentFraction: Perbill = Perbill::from_rational_approximation(1u32, 30 * DAYS);
    pub const SurchargeReward: Balance = 150 * MILLICENTS;
    pub const SignedClaimHandicap: u32 = 2;
    pub const MaxDepth: u32 = 32;
    pub const MaxValueSize: u32 = 16 * 1024;
    // The lazy deletion runs inside on_initialize.
    pub DeletionWeightLimit: Weight = AVERAGE_ON_INITIALIZE_RATIO *
    RuntimeBlockWeights::get().max_block;
    // The weight needed for decoding the queue should be less or equal than a fifth
    // of the overall weight dedicated to the lazy deletion.
    pub DeletionQueueDepth: u32 = ((DeletionWeightLimit::get() / (
            <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(1) -
            <Runtime as pallet_contracts::Config>::WeightInfo::on_initialize_per_queue_item(0)
        )) / 5) as u32;
    pub MaxCodeSize: u32 = 128 * 1024;
}

impl pallet_contracts::Config for Runtime {
    type Time = Timestamp;
    type Randomness = RandomnessCollectiveFlip;
    type Currency = Balances;
    type Event = Event;
    type RentPayment = ();
    type SignedClaimHandicap = SignedClaimHandicap;
    type TombstoneDeposit = TombstoneDeposit;
    type DepositPerContract = DepositPerContract;
    type DepositPerStorageByte = DepositPerStorageByte;
    type DepositPerStorageItem = DepositPerStorageItem;
    type RentFraction = RentFraction;
    type SurchargeReward = SurchargeReward;
    type MaxDepth = MaxDepth;
    type MaxValueSize = MaxValueSize;
    type WeightPrice = pallet_transaction_payment::Module<Self>;
    type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
    type ChainExtension = ();
    type DeletionQueueDepth = DeletionQueueDepth;
    type DeletionWeightLimit = DeletionWeightLimit;
    type MaxCodeSize = MaxCodeSize;
}
// /*** Pallet Contracts ***/

/*** Pallet Chips ***/
ord_parameter_types! {
    pub const W3FValidity: AccountId = AccountId::from(
        // 5CwARBdeFR8MJGvpHv7kaab2akiebDFGF9TDvRa5MimyGtEJ
        hex_literal::hex!("267bae633094eeb37b830d26ff4c6fa1e65ac162ef2e75ded0b8153f01beaa2f")
    );
}
impl pallet_chips::Config for Runtime {
    type Event = Event;
    type Balances = pallet_balances::Module<Runtime>;
    type ChipBalance = u128;
    type MasterAddress = W3FValidity;
    type WeightInfo = ();
}
/*** Pallet Chips ***/

/*** Pallet Bridge ***/
ord_parameter_types! {
    pub const BridgeOwner: AccountId = AccountId::from(
        // 5CwARBdeFR8MJGvpHv7kaab2akiebDFGF9TDvRa5MimyGtEJ
        hex_literal::hex!("267bae633094eeb37b830d26ff4c6fa1e65ac162ef2e75ded0b8153f01beaa2f")
    );
}
impl pallet_bridge::Config for Runtime {
    type Event = Event;
    type Balances = pallet_balances::Module<Runtime>;
    type OwnerAddress = BridgeOwner;
    type WeightInfo = ();
    type Assets = SubgameAssets;
}
/*** Pallet Chips ***/

/*** Pallet GameTemplate ***/
ord_parameter_types! {
    pub const TemplateOwner: AccountId = AccountId::from(
        // 5CwARBdeFR8MJGvpHv7kaab2akiebDFGF9TDvRa5MimyGtEJ
        hex_literal::hex!("267bae633094eeb37b830d26ff4c6fa1e65ac162ef2e75ded0b8153f01beaa2f")
    );
}
impl pallet_gametemplates::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type OwnerAddress = TemplateOwner;
}
/*** Pallet GameTemplate ***/

/*** Pallet GameCenter ***/
impl pallet_gamecenter::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type GuessHash = GameGuessHashModule;
}
/*** Pallet GameCenter ***/

/*** Pallet Game1: Guess Hash ***/
impl pallet_gametemplates_guess_hash::Config for Runtime {
    type Event = Event;
    type GameIndex = u32;
    type WeightInfo = ();
    type Chips = Chips;
}
/*** Pallet Game1: Guess Hash ***/

/***  scheduler ***/
// Define the types required by the Scheduler pallet.
parameter_types! {
    pub MaximumSchedulerWeight: Weight = 10_000_000;
    pub const MaxScheduledPerBlock: u32 = 50;
}
/// Configure the runtime's implementation of the Scheduler pallet.
impl pallet_scheduler::Config for Runtime {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    type MaxScheduledPerBlock = MaxScheduledPerBlock;
    type WeightInfo = ();
}
/***  scheduler ***/

/*** Pallet Stake ***/
ord_parameter_types! {
    pub const StakeOwner: AccountId = AccountId::from(
        // 5Cicojwqik9TqnsfX8o5ghgtHZi2jSDQDLQH2ophRjd2FxE2
        hex_literal::hex!("1cea52eeaf9fed98d4539330afcf8f10d501073cdf4561ee0bdf44f17fca234f")
    );
    pub const ImportOwner: AccountId = AccountId::from(
        hex_literal::hex!("f03bb9ee7cba9bf90724ac5bd90fcd9553969448dbd4cd3c88b0ee41a062c515")
    );
}
impl pallet_stake::Config for Runtime {
    type Event = Event;
    type Balances = pallet_balances::Module<Runtime>;
    type OwnerAddress = StakeOwner;
    type ImportAddress = ImportOwner;
    type WeightInfo = ();
    type Currency = Balances;
}
/*** Pallet Stake ***/

/*** Pallet Asset ***/
parameter_types! {
    pub const CommodityLimit: u128 =    1000000000000000000000;
    pub const UserCommodityLimit: u64 = 10000000000000000000;
}
impl pallet_nft::Config for Runtime {
/// The dispatch origin that is able to mint new instances of this type of commodity.
    type CommodityAdmin = EnsureRoot<AccountId>;
    /// The maximum number of this type of commodity that may exist (minted - burned).
    type CommodityLimit = CommodityLimit;
    /// The maximum number of this type of commodity that any single account may own.
    type UserCommodityLimit = UserCommodityLimit;
    type Event = Event;
}

ord_parameter_types! {
    pub const ModuleOwner: AccountId = AccountId::from(
        // 5CwARBdeFR8MJGvpHv7kaab2akiebDFGF9TDvRa5MimyGtEJ
        hex_literal::hex!("50eebb67d5888f999969633cdf644bf552500a18ecd156a972dd19fe7d4f1051")
    );
}
impl pallet_stake_nft::Config for Runtime {
    type ProgramId = u64;
    type PalletId = u64;
    type Balances = pallet_balances::Module<Runtime>;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type OwnerAddress = ModuleOwner;
    type Event = Event;
}

parameter_types! {
    pub const PalletIdPalletDemogame: u64 =    1;
    pub const PalletIdPalletManageCardInfo: u64 =    2;
    pub const PalletIdPalletCardFactory: u64 =    3;
    pub const PalletIdPalletNftExchange: u64 =    4;
    pub const PalletIdPalletGameRecharge: u64 = 5;
    pub const PalletIdPalletGameRechargePro: u64 = 6;
}

impl pallet_lease::Config for Runtime {
    type PalletId = u64;
    type UniqueAssets = SubgameNFT;
    type OwnerAddress = ModuleOwner;
    type Event = Event;
}

impl pallet_demogame::Config for Runtime {
    type PalletId = PalletIdPalletDemogame;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type Event = Event;
}

parameter_types! {
	pub const AssetDepositBase: Balance = 100 * MILLICENTS;
	pub const AssetDepositPerZombie: Balance = 1 * MILLICENTS;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10 * MILLICENTS;
	pub const MetadataDepositPerByte: Balance = 1 * MILLICENTS;
}
/// Configure the pallet_subgame_assets
impl pallet_subgame_assets::Config for Runtime {
	type Event = Event;
	type SGAssetBalance = u64;
	type AssetId = u32;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDepositBase = AssetDepositBase;
	type AssetDepositPerZombie = AssetDepositPerZombie;
	type StringLimit = StringLimit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type WeightInfo = pallet_subgame_assets::weights::SubstrateWeight<Runtime>;
}

/*** Pallet Swap ***/
impl pallet_swap::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type SwapId = u32;
    type Currency = Balances;
}

/*** Pallet Manage Card Info ***/
impl pallet_manage_card_info::Config for Runtime {
    type Event = Event;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type PalletId = PalletIdPalletManageCardInfo;
    type WeightInfo = ();
}

/*** Pallet card factory ***/
impl pallet_card_factory::Config for Runtime {
    type Event = Event;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type ManageCardInfo = ManageCardInfo;
    type PalletId = PalletIdPalletCardFactory;
    type WeightInfo = ();
}


ord_parameter_types! {
    pub const SeventhPlanetOwner: AccountId = AccountId::from(
        // 3iFsAZv22G5cTuGQzBpFKu58wWiQ7oFJuBcgkMsQPUirm2tu	
        hex_literal::hex!("2c46f54479f019745c0a06d5de89fa7c8b61005233ceb2cd81fcbb7bf334ac23")
    );
}
/*** Pallet Seventh Planet ***/
impl pallet_seventh_planet::Config for Runtime {
    type Event = Event;
    type OwnerAddress = SeventhPlanetOwner;
    type Balances = Balances;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type WeightInfo = ();
}

/*** Pallet Nft Exchange ***/
impl pallet_nft_exchange::Config for Runtime {
    type Event = Event;
    type Balances = Balances;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type PalletId = PalletIdPalletNftExchange;
    type WeightInfo = ();
}

/*** Pallet Game Recharge ***/
impl pallet_game_recharge::Config for Runtime {
	type Event = Event;
    type Assets = SubgameAssets;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type PalletId = PalletIdPalletGameRecharge;
	type WeightInfo = ();
}

/*** Pallet Game Recharge ***/
impl pallet_game_recharge_pro::Config for Runtime {
	type Event = Event;
    type Assets = SubgameAssets;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
    type PalletId = PalletIdPalletGameRechargePro;
	type WeightInfo = ();
}


/*** Pallet sonic racer ***/
ord_parameter_types! {
    pub const SonicRacerOwner: AccountId = AccountId::from(
        // 這私鑰有提供給廠商，只能用在測試
        // 0x6662e9bcbe30bf5da25d08d51c62ba8ac5dfcb0e8231e5b2432f53d8b1d3da42	
        // bounce crew own spice time mobile whisper collect chef crouch prevent gallery
        hex_literal::hex!("6662e9bcbe30bf5da25d08d51c62ba8ac5dfcb0e8231e5b2432f53d8b1d3da42")
    );
    pub const PackagePoolAddress: AccountId = AccountId::from(
        // 這私鑰有提供給廠商，只能用在測試
        // 0x6662e9bcbe30bf5da25d08d51c62ba8ac5dfcb0e8231e5b2432f53d8b1d3da42	
        // bounce crew own spice time mobile whisper collect chef crouch prevent gallery
        hex_literal::hex!("6662e9bcbe30bf5da25d08d51c62ba8ac5dfcb0e8231e5b2432f53d8b1d3da42")
    );
}
impl pallet_sonic_racer::Config for Runtime {
	type Event = Event;
    type Assets = SubgameAssets;
    type UniqueAssets = SubgameNFT;
    type OwnerAddress = SonicRacerOwner;
    type PackagePoolAddress = PackagePoolAddress;
    type Balances = Balances;
	type WeightInfo = ();
}

ord_parameter_types! {
    pub const TSPWhitelistOwner: AccountId = AccountId::from(
        // 3j9yeQo2gNaSMQJFMGAZn6P84sjADngxTHXbdcGZDAvx9v7w
        hex_literal::hex!("5405886b48c155875129e66571eae988b603bbaedb013f13ddab66fec7b42760")
    );
}
impl pallet_tspwhitelist::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type OwnerAddress = TSPWhitelistOwner;
}

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = opaque::Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
        Timestamp: pallet_timestamp::{Module, Call, Storage, Inherent},
        Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        TransactionPayment: pallet_transaction_payment::{Module, Storage},
        Sudo: pallet_sudo::{Module, Call, Config<T>, Storage, Event<T>},
        Contracts: pallet_contracts::{Module, Call, Config<T>, Storage, Event<T>},
        Scheduler: pallet_scheduler::{Module, Call, Storage, Event<T>},
        // Staking dependancies
		Babe: pallet_babe::{Module, Call, Storage, Config, Inherent, ValidateUnsigned},
        Grandpa: pallet_grandpa::{Module, Call, Storage, Config, Event, ValidateUnsigned},
		ImOnline: pallet_im_online::{Module, Call, Storage, Event<T>, ValidateUnsigned, Config<T>},
		Offences: pallet_offences::{Module, Call, Storage, Event},
		Staking: pallet_staking::{Module, Call, Config<T>, Storage, Event<T>},
		Authorship: pallet_authorship::{Module, Call, Storage, Inherent},
		AuthorityDiscovery: pallet_authority_discovery::{Module, Call},
		Indices: pallet_indices::{Module, Call, Storage, Config<T>, Event<T>},
		Session: pallet_session::{Module, Call, Storage, Event, Config<T>},
		Historical: session_historical::{Module},
        ElectionsPhragmen: pallet_elections_phragmen::{Module, Call, Storage, Event<T>, Config<T>},
		Treasury: pallet_treasury::{Module, Call, Storage, Config, Event<T>},
		Bounties: pallet_bounties::{Module, Call, Storage, Event<T>},
		Tips: pallet_tips::{Module, Call, Storage, Event<T>},
		// Democracy: pallet_democracy::{Module, Call, Storage, Config, Event<T>},
		Council: pallet_collective::<Instance1>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
		TechnicalCommittee: pallet_collective::<Instance2>::{Module, Call, Storage, Origin<T>, Event<T>, Config<T>},
		TechnicalMembership: pallet_membership::<Instance1>::{Module, Call, Storage, Event<T>, Config<T>},
		Multisig: pallet_multisig::{Module, Call, Storage, Event<T>},
		Recovery: pallet_recovery::{Module, Call, Storage, Event<T>},
		Proxy: pallet_proxy::{Module, Call, Storage, Event<T>},
		Utility: pallet_utility::{Module, Call, Event},
        Chips:  pallet_chips::{Module, Call, Storage, Event<T>},
        GameTemplates:	pallet_gametemplates::{Module, Call, Storage, Event<T>},
        GameCenter:	pallet_gamecenter::{Module, Call, Storage, Event<T>},
        GameGuessHashModule: pallet_gametemplates_guess_hash::{Module, Call, Storage, Event<T>},
        Bridge: pallet_bridge::{Module, Call, Storage, Event<T>},
        Stake: pallet_stake::{Module, Call, Storage, Event<T>},
        SubgameNFT: pallet_nft::{Module, Call, Storage, Event<T>},
        SubgameStakeNft: pallet_stake_nft::{Module, Call, Storage, Event<T>},
        Lease: pallet_lease::{Module, Call, Storage, Event<T>},
        DemoGame: pallet_demogame::{Module, Call, Storage, Event<T>},
        SubgameAssets: pallet_subgame_assets::{Module, Call, Storage, Event<T>},
        Swap: pallet_swap::{Module, Call, Storage, Event<T>},
        ManageCardInfo: pallet_manage_card_info::{Module, Call, Storage, Event<T>},
        CardFactory: pallet_card_factory::{Module, Call, Storage, Event<T>},
        SeventhPlanet: pallet_seventh_planet::{Module, Call, Storage, Event<T>},
        NftExchange: pallet_nft_exchange::{Module, Call, Storage, Event<T>},
        GameRecharge: pallet_game_recharge::{Module, Call, Storage, Event<T>},
        GameRechargePro: pallet_game_recharge_pro::{Module, Call, Storage, Event<T>},
        SonicRacer: pallet_sonic_racer::{Module, Call, Storage, Event<T>},
        TspWhitelist: pallet_tspwhitelist::{Module, Call, Storage, Event<T>},
    }
);

/// The address format for describing accounts.
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
    frame_system::CheckSpecVersion<Runtime>,
    frame_system::CheckTxVersion<Runtime>,
    frame_system::CheckGenesis<Runtime>,
    frame_system::CheckEra<Runtime>,
    frame_system::CheckNonce<Runtime>,
    frame_system::CheckWeight<Runtime>,
    pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
    Runtime,
    Block,
    frame_system::ChainContext<Runtime>,
    Runtime,
    AllModules,
>;

pub const PRIMARY_PROBABILITY: (u64, u64) = (1, 4);
impl_runtime_apis! {
    impl sp_api::Core<Block> for Runtime {
        fn version() -> RuntimeVersion {
            VERSION
        }

        fn execute_block(block: Block) {
            Executive::execute_block(block)
        }

        fn initialize_block(header: &<Block as BlockT>::Header) {
            Executive::initialize_block(header)
        }
    }

    impl sp_api::Metadata<Block> for Runtime {
        fn metadata() -> OpaqueMetadata {
            Runtime::metadata().into()
        }
    }

    impl sp_block_builder::BlockBuilder<Block> for Runtime {
        fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
            Executive::apply_extrinsic(extrinsic)
        }

        fn finalize_block() -> <Block as BlockT>::Header {
            Executive::finalize_block()
        }

        fn inherent_extrinsics(data: sp_inherents::InherentData) ->
            Vec<<Block as BlockT>::Extrinsic> {
            data.create_extrinsics()
        }

        fn check_inherents(
            block: Block,
            data: sp_inherents::InherentData,
        ) -> sp_inherents::CheckInherentsResult {
            data.check_extrinsics(&block)
        }

        fn random_seed() -> <Block as BlockT>::Hash {
            RandomnessCollectiveFlip::random_seed()
        }
    }

    impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
        fn validate_transaction(
            source: TransactionSource,
            tx: <Block as BlockT>::Extrinsic,
        ) -> TransactionValidity {
            Executive::validate_transaction(source, tx)
        }
    }

    impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
        fn offchain_worker(header: &<Block as BlockT>::Header) {
            Executive::offchain_worker(header)
        }
    }

    impl sp_consensus_babe::BabeApi<Block> for Runtime {
        fn configuration() -> sp_consensus_babe::BabeGenesisConfiguration {
            sp_consensus_babe::BabeGenesisConfiguration {
                slot_duration: Babe::slot_duration(),
                epoch_length: EpochDuration::get(),
                c: PRIMARY_PROBABILITY,
                genesis_authorities: Babe::authorities(),
                randomness: Babe::randomness(),
                allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryPlainSlots,
            }
        }

        fn current_epoch_start() -> sp_consensus_babe::Slot {
            Babe::current_epoch_start()
        }

        fn current_epoch() -> sp_consensus_babe::Epoch {
            Babe::current_epoch()
        }

        fn next_epoch() -> sp_consensus_babe::Epoch {
            Babe::next_epoch()
        }

        fn generate_key_ownership_proof(
            _slot: sp_consensus_babe::Slot,
            authority_id: sp_consensus_babe::AuthorityId,
        ) -> Option<sp_consensus_babe::OpaqueKeyOwnershipProof> {
            use codec::Encode;

            Historical::prove((sp_consensus_babe::KEY_TYPE, authority_id))
                .map(|p| p.encode())
                .map(sp_consensus_babe::OpaqueKeyOwnershipProof::new)
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            equivocation_proof: sp_consensus_babe::EquivocationProof<<Block as BlockT>::Header>,
            key_owner_proof: sp_consensus_babe::OpaqueKeyOwnershipProof,
            ) -> Option<()> {
            let key_owner_proof = key_owner_proof.decode()?;

            Babe::submit_unsigned_equivocation_report(
                equivocation_proof,
                key_owner_proof,
                )
        }
    }

	impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityDiscoveryId> {
			AuthorityDiscovery::authorities()
		}
	}

    impl sp_session::SessionKeys<Block> for Runtime {
        fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
            opaque::SessionKeys::generate(seed)
        }

        fn decode_session_keys(
            encoded: Vec<u8>,
        ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
            opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
        }
    }

    impl fg_primitives::GrandpaApi<Block> for Runtime {
        fn grandpa_authorities() -> GrandpaAuthorityList {
            Grandpa::grandpa_authorities()
        }

        fn submit_report_equivocation_unsigned_extrinsic(
            _equivocation_proof: fg_primitives::EquivocationProof<
                <Block as BlockT>::Hash,
                NumberFor<Block>,
            >,
            _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
        ) -> Option<()> {
            None
        }

        fn generate_key_ownership_proof(
            _set_id: fg_primitives::SetId,
            _authority_id: GrandpaId,
        ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
            // NOTE: this is the only implementation possible since we've
            // defined our key owner proof type as a bottom type (i.e. a type
            // with no values).
            None
        }
    }

    impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
        fn account_nonce(account: AccountId) -> Index {
            System::account_nonce(account)
        }
    }

    impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
        for Runtime {
        fn query_info(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
            TransactionPayment::query_info(uxt, len)
        }
        fn query_fee_details(
            uxt: <Block as BlockT>::Extrinsic,
            len: u32,
        ) -> pallet_transaction_payment::FeeDetails<Balance> {
            TransactionPayment::query_fee_details(uxt, len)
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    impl frame_benchmarking::Benchmark<Block> for Runtime {
        fn dispatch_benchmark(
            config: frame_benchmarking::BenchmarkConfig
        ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
            use frame_benchmarking::{Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

            use frame_system_benchmarking::Module as SystemBench;
            impl frame_system_benchmarking::Config for Runtime {}

            let whitelist: Vec<TrackedStorageKey> = vec![
                // Block Number
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac")
                    .to_vec().into(),
                // Total Issuance
                hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80")
                    .to_vec().into(),
                // Execution Phase
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a")
                    .to_vec().into(),
                // Event Count
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850")
                    .to_vec().into(),
                // System Events
                hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7")
                    .to_vec().into(),
            ];

            let mut batches = Vec::<BenchmarkBatch>::new();
            let params = (&config, &whitelist);

            add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
            add_benchmark!(params, batches, pallet_balances, Balances);
            add_benchmark!(params, batches, pallet_timestamp, Timestamp);
            add_benchmark!(params, batches, pallet_stake, Stake);
            add_benchmark!(params, batches, pallet_swap, Swap);
            add_benchmark!(params, batches, pallet_bridge, Bridge);

            if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
            Ok(batches)
        }
    }

    /*** Pallet Contracts ***/
    impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance, BlockNumber>
    for Runtime
    {
        fn call(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
            gas_limit: u64,
            input_data: Vec<u8>,
        ) -> pallet_contracts_primitives::ContractExecResult {
            Contracts::bare_call(origin, dest, value, gas_limit, input_data)
        }

        fn get_storage(
            address: AccountId,
            key: [u8; 32],
        ) -> pallet_contracts_primitives::GetStorageResult {
            Contracts::get_storage(address, key)
        }

        fn rent_projection(
            address: AccountId,
        ) -> pallet_contracts_primitives::RentProjectionResult<BlockNumber> {
            Contracts::rent_projection(address)
        }
    }
    /*** Pallet Contracts ***/
}
