# pallet-lease

## description
A list of leasable modules is defined in pallet-lease, and users can obtain module usage rights by staking nft tokens.

## Getting Started

### Importing a Pallet Crate

`runtime/Cargo.toml`

```
[dependencies]
...
// Add this code
pallet-lease = { path = '../pallets/lease', default-features = false, version = '3.0.0' }
```

### Configure the Pallet

`runtime/src/lib.rs`

```
// Add this code
pub use pallet_lease;


// Add this code
ord_parameter_types! {
    pub const W3FValidity: AccountId = AccountId::from(
        // 5CwARBdeFR8MJGvpHv7kaab2akiebDFGF9TDvRa5MimyGtEJ
        hex_literal::hex!("267bae633094eeb37b830d26ff4c6fa1e65ac162ef2e75ded0b8153f01beaa2f")
    );
}
impl pallet_lease::Config for Runtime {
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
        Chips: pallet_lease::{Module, Call, Storage, Event<T>},
	}
);
```

## Other Module Use Lease

##### use lease trait
```rust
    pub trait Config: frame_system::Config {
        ...
        // add 
        type Lease: Lease<Self::AccountId, NftId<Self>>;
        ...
    }

```

##### check permission in module
```rust
    let is_ok = T::Lease::check_authority(T::PalletId::get(), sender.clone())?;
    ensure!(is_ok == true, Error::<T>::PermissionDenied);
```

## Test Pallet

```
cargo test
```

## Documentation

```
cargo doc --open --package pallet-lease
```