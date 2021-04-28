#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

impl crate::WeightInfo for () {
	fn create_game() -> Weight {
		(500_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn play_game() -> Weight {
		(500_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn on_finalize(count: u32) -> Weight {
		(50_000_000 as Weight)
			.saturating_add(DbWeight::get().reads(count as Weight))
			.saturating_add(DbWeight::get().writes(count as Weight))
	}
}