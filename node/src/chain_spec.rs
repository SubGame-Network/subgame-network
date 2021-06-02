use hex_literal::hex;
use sc_service::ChainType;
use sc_service::Properties;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::crypto::UncheckedInto;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use subgame_runtime::ContractsConfig;
use subgame_runtime::{
    AccountId, AuraConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
    SystemConfig, WASM_BINARY,
};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

/// Properties for Subgame.
pub fn subgame_properties() -> Properties {
    let mut properties = Properties::new();
    properties.insert("ss58Format".into(), 27.into());
    properties.insert("tokenDecimals".into(), vec![10].into());
    properties.insert("tokenSymbol".into(), vec!["SGB"].into());
    properties
}
/// Properties for Subgame.
pub fn subgame_mainnet_properties() -> Properties {
    let mut properties = Properties::new();
    properties.insert("ss58Format".into(), 27.into());
    properties.insert("tokenDecimals".into(), vec![10].into());
    properties.insert("tokenSymbol".into(), vec!["SGB"].into());
    properties
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary =
        WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "subgame_dev",
        // ID
        "subgame_dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(subgame_properties()),
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary =
        WASM_BINARY.ok_or_else(|| "Development wasm binary not available".to_string())?;
    Ok(ChainSpec::from_genesis(
        // Name
        "subgame_testnet",
        // ID
        "subgame_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    (
                        hex!["50a03202347d4cb254b62c476aef48b1cf0f44603913674b805d19e1a8987208"]
                            .unchecked_into(),
                        hex!["e3bda27d551a3fbff7e58f4388ecd33ffccdf877c55bf559e32f3076a61beb45"]
                            .unchecked_into(),
                    ),
                    (
                        hex!["088b874d19c72096c1a5b1b7789592a980d712a79c5e5d4b5493f1b4bb3d6151"]
                            .unchecked_into(),
                        hex!["7e41726e3d43b84d44c02e92daa7ab51207bfdd0d3202af5aa4ef8505e7e1f51"]
                            .unchecked_into(),
                    ),
                ],
                // Sudo account
                hex!["f03bb9ee7cba9bf90724ac5bd90fcd9553969448dbd4cd3c88b0ee41a062c515"].into(),
                // Pre-funded accounts
                vec![
                    hex!["f03bb9ee7cba9bf90724ac5bd90fcd9553969448dbd4cd3c88b0ee41a062c515"].into(),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(subgame_properties()),
        // Extensions
        None,
    ))
}

pub fn mainnet_config() -> Result<ChainSpec, String> {
    let wasm_binary =
        WASM_BINARY.ok_or_else(|| "Main net wasm binary not available".to_string())?;
    Ok(ChainSpec::from_genesis(
        // Name
        "subgame",
        // ID
        "subgame",
        ChainType::Live,
        move || {
            mainnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    (
                        hex!["cc5cca7c16f2e9e8200feb5c37bdf9477b0a1655121131cc12c4d8287d826a3c"]
                            .unchecked_into(),
                        hex!["65e7a65555736015ade3d367b679fbb97ea10103220c97395e6b28f8ed5fdc42"]
                            .unchecked_into(),
                    ),
                    (
                        hex!["aae8e7cf280cacf0b1c192f07ea56f6855b06b2ed60b673b96c20a5fc2a27f05"]
                            .unchecked_into(),
                        hex!["36d4de857c1627d8db85947f86b9d8ea9ad6a58600eaf6b2266e3f2fb8298f5f"]
                            .unchecked_into(),
                    ),
                ],
                // Sudo account
                hex!["f03bb9ee7cba9bf90724ac5bd90fcd9553969448dbd4cd3c88b0ee41a062c515"].into(),
                // Pre-funded accounts
                vec![
                    hex!["f03bb9ee7cba9bf90724ac5bd90fcd9553969448dbd4cd3c88b0ee41a062c515"].into(),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(subgame_mainnet_properties()),
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 5000000000000000000))
                .collect(),
        }),
        pallet_aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        }),
        pallet_sudo: Some(SudoConfig {
            // Assign network admin rights.
            key: root_key,
        }),
        /*** Pallet Contracts ***/
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println,
                ..Default::default()
            },
        }),
        /*** Pallet Contracts ***/
    }
}

/// Configure initial storage state for FRAME modules.
fn mainnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        frame_system: Some(SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 5000000000000000000))
                .collect(),
        }),
        pallet_aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        }),
        pallet_sudo: Some(SudoConfig {
            // Assign network admin rights.
            key: root_key,
        }),
        /*** Pallet Contracts ***/
        pallet_contracts: Some(ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println,
                ..Default::default()
            },
        }),
        /*** Pallet Contracts ***/
    }
}
