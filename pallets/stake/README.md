# pallet-stake

## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-stake = { path = '../pallets/stake', default-features = false, version = '3.0.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use pallet_stake;


// Add this code
ord_parameter_types! {
    pub const StakeOwner: AccountId = AccountId::from(
        // 5CwARBdeFR8MJGvpHv7kaab2akiebDFGF9TDvRa5MimyGtEJ
        hex_literal::hex!("267bae633094eeb37b830d26ff4c6fa1e65ac162ef2e75ded0b8153f01beaa2f")
    );
}
impl pallet_stake::Config for Runtime {
    type Event = Event;
    type Balances = pallet_balances::Module<Runtime>;
    type OwnerAddress = StakeOwner;
    type WeightInfo = ();
    type Currency = Balances;
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
        ...
        
        // Add this code
        Stake: pallet_stake::{Module, Call, Storage, Event<T>},
	}
);
```

## Test Pallet

```
cargo test
```

## Documentation

```
cargo doc --open --package pallet-stake
```