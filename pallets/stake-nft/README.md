# pallet-stake-nft

## description
After providing users to stake SGB, they can get an nft token, and they can use special functions with nft token. SGB will be returned through redemption, and nft token will be burned at the same time. The module will provide different stake amount schemes and different valid periods. When the stake expires, nft token can no longer be used for special functions, SGB can be returned through redemption, and nft token will be burned.

## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-stake-nft = { path = '../pallets/stake-nft', default-features = false, version = '3.0.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use pallet_stake_nft;


// Add this code
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


construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...
        
        // Add this code
        SubgameNFT: pallet_stake_nft::{Module, Call, Storage, Event<T>},
	}
);
```

## Test Pallet

```
cargo test
```

## Documentation

```
cargo doc --open --package pallet-stake-nft
```