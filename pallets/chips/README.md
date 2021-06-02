# pallet-chips

## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-chips = { path = '../pallets/chips', default-features = false, version = '3.0.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use pallet_chips;


// Add this code
ord_parameter_types! {
    pub const W3FValidity: AccountId = AccountId::from(
        // 5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL
        hex_literal::hex!("1cbd2d43530a44705ad088af313e18f80b53ef16b36177cd4b77b846f2a5f07c")
    );
}
impl pallet_chips::Config for Runtime {
    type Event = Event;
    type Balances = pallet_balances::Module<Runtime>;
    type ChipBalance = u128;
    type MasterAddress = W3FValidity;
    type WeightInfo = ();
}


construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...
        
        // Add this code
        Chips: pallet_chips::{Module, Call, Storage, Event<T>},
	}
);
```

## Test Pallet

```
cargo test
```

## Documentation

```
cargo doc --open --package pallet-chips
```