# pallet-gamecenter

## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-gamecenter = { path = '../pallets/gamecenter', default-features = false, version = '3.0.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use pallet_gamecenter;


// Add this code
impl pallet_gamecenter::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type GuessHash = GameGuessHashModule;
}


construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...
        
        // Add this code
        GameCenter:	pallet_gamecenter::{Module, Call, Storage, Event<T>},
	}
);
```

## Test Pallet

```
cargo test
```

## Documentation

```
cargo doc --open --package pallet-gamecenter
```