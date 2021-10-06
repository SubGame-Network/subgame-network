# pallet-nft

## description
A unique asset (NFT) interface and a Substrate FRAME implementation optimized for commodity assets.

## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-nft = { path = '../pallets/nft', default-features = false, version = '3.0.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use pallet_nft;


// Add this code
parameter_types! {
    pub const CommodityLimit: u128 =    1000000000000000000000;
    pub const UserCommodityLimit: u64 = 10000000000000000000;
}
impl pallet_nft::Config for Runtime {
    type CommodityAdmin = EnsureRoot<AccountId>;
    type CommodityLimit = CommodityLimit;
    type UserCommodityLimit = UserCommodityLimit;
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
        SubgameNFT: pallet_nft::{Module, Call, Storage, Event<T>},
	}
);
```

## Test Pallet

```
cargo test
```

## Documentation

```
cargo doc --open --package pallet-nft
```