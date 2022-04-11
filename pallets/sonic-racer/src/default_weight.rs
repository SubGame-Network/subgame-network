#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    // 銷售相關
    fn set_applybuy_whitelist() -> Weight;
    fn user_applybuy() -> Weight;

    // 盲盒相關
    fn open_blindbox() -> Weight;

    fn props_fusion() -> Weight;

    fn asset_package() -> Weight;
    fn asset_unpackage() -> Weight;
    fn set_game() -> Weight;
    fn create_platform() -> Weight;
    fn update_platform() -> Weight;
    fn platform_change_admin() -> Weight;
    fn recharge() -> Weight;
    fn withdraw() -> Weight;
}

impl crate::WeightInfo for () {
    fn set_applybuy_whitelist() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn user_applybuy() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(505 as Weight))
            .saturating_add(DbWeight::get().writes(505 as Weight))
    }
    fn open_blindbox() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn props_fusion() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn asset_package() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn asset_unpackage() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn set_game() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn create_platform() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn update_platform() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn platform_change_admin() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn recharge() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    fn withdraw() -> Weight {
        (10_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
}
