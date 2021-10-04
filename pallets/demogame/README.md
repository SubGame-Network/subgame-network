# pallet-demogame

## description
This module is a special pallet, and you need to use `stake-nft` to obtain access rights (this is a simulated pallet)


## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-demogame = { path = '../pallets/demogame', default-features = false, version = '3.0.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use pallet_demogame;


// Add this code
impl pallet_demogame::Config for Runtime {
    type PalletId = PalletIdPalletDemogame;
    type UniqueAssets = SubgameNFT;
    type Lease = Lease;
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
        DemoGame: pallet_demogame::{Module, Call, Storage, Event<T>},
	}
);

```