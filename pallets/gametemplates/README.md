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
        // 5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL
        hex_literal::hex!("1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c")
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