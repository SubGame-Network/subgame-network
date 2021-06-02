# pallet-gametemplates-guess-hash

## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-gametemplates-guess-hash = { path = '../pallets/gametemplates-guess-hash', default-features = false, version = '3.0.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use pallet_gametemplates_guess_hash;


// Add this code
impl pallet_gametemplates_guess_hash::Config for Runtime {
    type Event = Event;
    type GameIndex = u32;
    type WeightInfo = ();
    type Chips = Chips;
}


construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...
        
        // Add this code
        GameGuessHashModule: pallet_gametemplates_guess_hash::{Module, Call, Storage, Event<T>},
	}
);
```

## Test Pallet

```
cargo test
```

## Documentation

```
cargo doc --open --package pallet-gametemplates-guess-hash
```