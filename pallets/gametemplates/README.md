# pallet-gametemplates

## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-gametemplates = { path = '../pallets/gametemplates', default-features = false, version = '3.0.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use pallet_gametemplates;


// Add this code
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


construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...
        
        // Add this code
        GameTemplates: pallet_gametemplates::{Module, Call, Storage, Event<T>},
	}
);
```

## Test Pallet

```
cargo test
```

## Documentation

```
cargo doc --open --package pallet-gametemplates
```