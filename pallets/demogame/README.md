# pallet-demogame

## description
We will provide this game A pallet to users for lease. This is a sample pallet. There will be more pallets that can be leased in the future.
This module is a special pallet, and you need to use `stake-nft` to obtain access rights (this is a simulated pallet)

#### demo()
If you have permission, call `demo` function will make `CallSuccess` + 1

```rust
demo(origin) -> dispatch::DispatchResult
```

## storage
#### CallSuccess
The number of successful call functions is recorded here

```rust
 pub CallSuccess get(fn call_success): map hasher(blake2_128_concat) T::AccountId=> u64 = 0;
```

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