use hex_literal::hex;
use sc_service::ChainType;
use sc_service::Properties;
use sp_consensus_babe::{AuthorityId as BabeId};
use sp_runtime::{Perbill};
use sp_core::crypto::UncheckedInto;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use subgame_runtime::ContractsConfig;
use subgame_runtime::{
    opaque::SessionKeys,
    AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
    StakerStatus, Balance,
	SessionConfig, StakingConfig, ImOnlineConfig,
    SystemConfig, WASM_BINARY,
	IndicesConfig, CouncilConfig, TechnicalCommitteeConfig,
};

fn session_keys(
    babe: BabeId,
    grandpa: GrandpaId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
    SessionKeys { babe, grandpa, im_online, authority_discovery}
}

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

/// Generate an Babe authority key.
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId,) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
        get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
		get_from_seed::<AuthorityDiscoveryId>(s),
    )
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
                    (get_account_id_from_seed::<sr25519::Public>("Alice"), 5000000000000000000),
                    (get_account_id_from_seed::<sr25519::Public>("Bob"), 5000000000000000000),
                    (get_account_id_from_seed::<sr25519::Public>("Alice//stash"), 5000000000000000000),
                    (get_account_id_from_seed::<sr25519::Public>("Bob//stash"), 5000000000000000000),
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
        "SubGame Staging",
        // ID
        "subgame_staging",
        ChainType::Live,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    (
                        // 5FpfTNAjx3yjU8P6N74uwGj19bpuEXrHTHRm1pWdxAw8Pn65	
                        hex!["a63b69bded2ac349e87634116fe96ae1cd1e700f851317aee1a00f0745ec6c1a"].into(),
                        // 5FF5CH22pskNGB8d7r7DokSmyvXZJDXSqE4CF3rwFn1wYMP4	
                        hex!["8c9cfc192f256cf6ef76101827353c0f0e28d50ced6aef7a7677fac4f2017825"].into(),
                        // 5GEXjMYCYuogszM2WQnnNKA6bbzCNTKaAAY3BFmNkEsEwgsi	
                        hex!["b86f4ce48d4d810d7bdb5fe76397aecd59d5d9f561fee8963eb71ac989ecd643"].unchecked_into(),
                        // 5F4VQxErPWLFepqABApZa5uqt9n4XgairQR3M2py2fJAyzzy	
                        hex!["848acfeec0d4aae8baf2169724351cfd990863bd5bf0c00e35ac2c60f404fa1f"].unchecked_into(),
                        // 5GEXjMYCYuogszM2WQnnNKA6bbzCNTKaAAY3BFmNkEsEwgsi	
                        hex!["b86f4ce48d4d810d7bdb5fe76397aecd59d5d9f561fee8963eb71ac989ecd643"].unchecked_into(),
                        // 5GEXjMYCYuogszM2WQnnNKA6bbzCNTKaAAY3BFmNkEsEwgsi	
                        hex!["b86f4ce48d4d810d7bdb5fe76397aecd59d5d9f561fee8963eb71ac989ecd643"].unchecked_into(),
                    ),
                    (
                        // 5EyJyiFeYtDxdzXbk62wTZSnHyn3Rwhbc56oELNzJVTG2rRY	
                        hex!["8097750cd4845d1e9b5ad167845dfceb43511271ea3ac966f082e7ba003aa87c"].into(),
                        // 5ERJW7v4fSvv91HyuE4Uq5RCaP44ms2FR1pzDKadyVUT8VHY	
                        hex!["682e00bb24c69cd815f4e09dc345a78f3db1191654650bb1e249fbe87f009764"].into(),
                        // 5GNegtwtqxUkjt6JRDy3tbjqWzpaPsW1yjURRr4nyNn4kftb	
                        hex!["bea0af5cbdf831165065bc48495aefe3d6acfcce8ff3b67a1051cd74eecaa86a"].unchecked_into(),
                        // 5DXHpnbhLVjhuE1JGH86q8h3XLGeBmde7pXpGbuqyrPpu3dB	
                        hex!["4082f57de59206d7fcfb839acaa109a63e7860430e0fdaf2edfacdfa2d7cee04"].unchecked_into(),
                        // 5GNegtwtqxUkjt6JRDy3tbjqWzpaPsW1yjURRr4nyNn4kftb	
                        hex!["bea0af5cbdf831165065bc48495aefe3d6acfcce8ff3b67a1051cd74eecaa86a"].unchecked_into(),
                        // 5GNegtwtqxUkjt6JRDy3tbjqWzpaPsW1yjURRr4nyNn4kftb	
                        hex!["bea0af5cbdf831165065bc48495aefe3d6acfcce8ff3b67a1051cd74eecaa86a"].unchecked_into(),
                    ),
                ],
                // Sudo account
                hex!["f03bb9ee7cba9bf90724ac5bd90fcd9553969448dbd4cd3c88b0ee41a062c515"].into(),
                // Pre-funded accounts
                vec![
                    (hex!["f03bb9ee7cba9bf90724ac5bd90fcd9553969448dbd4cd3c88b0ee41a062c515"].into(), 4999998000000000000),  // 499999800
                    // // 5FpfTNAjx3yjU8P6N74uwGj19bpuEXrHTHRm1pWdxAw8Pn65	
                    (hex!["a63b69bded2ac349e87634116fe96ae1cd1e700f851317aee1a00f0745ec6c1a"].into(), 1000000000000),    // 100
                    // 5EyJyiFeYtDxdzXbk62wTZSnHyn3Rwhbc56oELNzJVTG2rRY	
                    (hex!["8097750cd4845d1e9b5ad167845dfceb43511271ea3ac966f082e7ba003aa87c"].into(), 1000000000000),    // 100
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
        "SubGame",
        // ID
        "subgame",
        ChainType::Live,
        move || {
            mainnet_genesis(
                 wasm_binary,
                // Initial PoA authorities
                vec![
                    (
                        // 5FWWuhHThTUSL97FBpXU9EwobbZ6YZqCs8ryRGDvzAqhfzYF	
                        hex!["98643a2c1477740412cf7b2d7203443626b191523df56ba35ec4a4c5b56b814c"].into(),
                        // 5E9k7QkUua2GtrJpiG6WD69YU3qjVbMzzmutbstahkJmo3ZS	
                        hex!["5c50483925073024e9457f6df77e6a66bd22eb80f9bd0ffd815df1aa969ed04c"].into(),
                        // 5FFKTjUrtLFARzXtXVB2Wy12w4A7PezpR8VKMSyEPFtG9y86	
                        hex!["8ccd0291b5608d702cfe1a7d37c72167009385460ed2a609f743ec8b31afa709"].unchecked_into(),
                        // 5DUEfARox7DXNTef2d9cpanotu2hVhHXjctZJmWWAD99JY2B	
                        hex!["3e2e90b6a429f3b045e884f7dc1118da906455295764f4507dd0a733abd02f41"].unchecked_into(),
                        // 5FFKTjUrtLFARzXtXVB2Wy12w4A7PezpR8VKMSyEPFtG9y86	
                        hex!["8ccd0291b5608d702cfe1a7d37c72167009385460ed2a609f743ec8b31afa709"].unchecked_into(),
                        // 5FFKTjUrtLFARzXtXVB2Wy12w4A7PezpR8VKMSyEPFtG9y86	
                        hex!["8ccd0291b5608d702cfe1a7d37c72167009385460ed2a609f743ec8b31afa709"].unchecked_into(),
                    ),
                    (
                        // 5Gitx5RkseoZeGcyLmmTA48GZBf2WE3TD4TnrWWgtgm1VJFK	
                        hex!["ce119a358c2c5e0e1e52301e77c7997dee651ee67a436b03d60252dc5494c03a"].into(),
                        // 5DPRbHyqXzeUcWV9aPWtaFHqB4xBAYrqpZbUscm93wHdgHcg	
                        hex!["3a832499549464d61c4b6ca47e36de8ed2f0fdfd35ac9b0214b5186d13755e56"].into(),
                        // 5DfY3Uwmh35NcXUYu63oU8ktPJTZU9WiPQ1JQ3NR7gNqHjwX	
                        hex!["46ccc9886ad9e9afffda6719a8d395d00231f71a7fb34d7dc2c4a777c70f8b74"].unchecked_into(),
                        // 5EfKCjVEJVGWyusmj4CMZo3epGoGqzMfjjkqExghuUVPwwbJ	
                        hex!["72ddcbb2ef0324ba25f97e07fc9c214e4e48c23dbc82731cc4146903c89bf9d8"].unchecked_into(),
                        // 5DfY3Uwmh35NcXUYu63oU8ktPJTZU9WiPQ1JQ3NR7gNqHjwX	
                        hex!["46ccc9886ad9e9afffda6719a8d395d00231f71a7fb34d7dc2c4a777c70f8b74"].unchecked_into(),
                        // 5DfY3Uwmh35NcXUYu63oU8ktPJTZU9WiPQ1JQ3NR7gNqHjwX	
                        hex!["46ccc9886ad9e9afffda6719a8d395d00231f71a7fb34d7dc2c4a777c70f8b74"].unchecked_into(),
                    ),
                    (
                        // 5CAmCtwtLkmBgwNhmkQhmktSh9JRcwKfRv8Ee3m5dJxu1LjD
                        hex!["049ebece3fe3525306fb16dd804c959684278eec692158320a4c94b11f847e39"].into(),
                        // 5EM2XHuR8HRoG2KgWY4eyGrmidqx12RqTgY9dRmXNdN7nEuh
                        hex!["64eb3a7d28820d81e96aebefad103425adf2187ff1a1626556518d9a859f7201"].into(),
                        // 5HEiHuGbFqFroAQQinYQwZ8tbtA7gPpxN5rwrFGuj7urEsHB
                        hex!["e4cf15f9766efdfd7a1333096b69d167272c9f6485e0846fa0d4848a424cc013"].unchecked_into(),
                        // 5EFAqA3V1pkT2FvXQRwzeBvqLqBkwtBYeHeSSdmjgKiKfFCo
                        hex!["6073b929cf698e7aaaadbd8d02221936e0d5ef060c962aa3363fb64ac41b4503"].unchecked_into(),
                        // 5HEiHuGbFqFroAQQinYQwZ8tbtA7gPpxN5rwrFGuj7urEsHB
                        hex!["e4cf15f9766efdfd7a1333096b69d167272c9f6485e0846fa0d4848a424cc013"].unchecked_into(),
                        // 5HEiHuGbFqFroAQQinYQwZ8tbtA7gPpxN5rwrFGuj7urEsHB
                        hex!["e4cf15f9766efdfd7a1333096b69d167272c9f6485e0846fa0d4848a424cc013"].unchecked_into(),
                    ),
                    (
                        // 5EgxnoyYB1EvvWUNKvpJFxCG8fsg34f4G2PzkybSiG6DR5Dr
                        hex!["741f927195ace2d31c4f625cf89556bfa06a764780a1cd7ef333c8a927ab9401"].into(),
                        // 5Gedtojenv3hFW5UTsdzu1tX25AJL4F1VMjwtpoNyzL3r9vp
                        hex!["cad1efba8560b51fcf0062212022dec524bd22d193d95dc0f48c0a52091ceb4e"].into(),
                        // 5E4866nxQfma332tpDYT8vV3B6FwHsKTABijdvfNteLcKNvo
                        hex!["5806cad365b0e59c7365b02f8d809f756bf19e1a49db2f008dd1738d5328844e"].unchecked_into(),
                        // 5CibZaL865e2RUYFEQ4Q5TsT9jMBJzERWxcDPHN2fm97WA1m
                        hex!["1ce622ae9d22f9bc3a5d345adc1831da02cedf52284cc14d6bf33048e4e8f5f0"].unchecked_into(),
                        // 5E4866nxQfma332tpDYT8vV3B6FwHsKTABijdvfNteLcKNvo
                        hex!["5806cad365b0e59c7365b02f8d809f756bf19e1a49db2f008dd1738d5328844e"].unchecked_into(),
                        // 5E4866nxQfma332tpDYT8vV3B6FwHsKTABijdvfNteLcKNvo
                        hex!["5806cad365b0e59c7365b02f8d809f756bf19e1a49db2f008dd1738d5328844e"].unchecked_into(),
                    ),
                    (
                        // 5CGDWDTdRh28dQ31owjGjuphbhjta95hpRVRpuXdGDe1sZy6
                        hex!["08c78058ff97641449aa5e0e018d777135324b83a81c704d2da1f6a1ad25df22"].into(),
                        // 5CqQJ16obtE3CbEoPpQEZHxjDuUuqHWKmnMfMseh6z1Aredn
                        hex!["2216eb8f9100f369fed1f88b467f8aafdc3239c08f7e18aee8d8f4baf1b99f2f"].into(),
                        // 5FxP1VaCLwhBnQkjDPjft51i8NQ1HNQiU7mgkczLDPqo3ta9
                        hex!["ac1e038cbf41b92fec53c1079a91c622f5a9195f0e3dc691282df2ac4c7cb070"].unchecked_into(),
                        // 5CTxojsNU6odisAXykintiBZEdgZ22iNvhKUaQEF4RAzYx5J
                        hex!["11bcf80429aadd0903ccbb6bf68c55c34fa299dd0de9cf4ad4c85f1ea7e86758"].unchecked_into(),
                        // 5FxP1VaCLwhBnQkjDPjft51i8NQ1HNQiU7mgkczLDPqo3ta9
                        hex!["ac1e038cbf41b92fec53c1079a91c622f5a9195f0e3dc691282df2ac4c7cb070"].unchecked_into(),
                        // 5FxP1VaCLwhBnQkjDPjft51i8NQ1HNQiU7mgkczLDPqo3ta9
                        hex!["ac1e038cbf41b92fec53c1079a91c622f5a9195f0e3dc691282df2ac4c7cb070"].unchecked_into(),
                    ),
                    (
                        // 5E1VUev4ym8QBDVE9Wp2oReWuYCfkJJBLnrFxdytqHc7yH5q
                        hex!["56050eb15aa425e8a502477d7cc0e4bd4b5e55b67a0b340ea27e437ed4595302"].into(),
                        // 5Do6AToGKuzaJgyVTbjm8ksjSAkkJEniijzoc8ahDxwTijJA
                        hex!["4c8fa1bdb43bdeeb682a8bfd80ae49562faa0b76366035ab1f9b0fa7bb899347"].into(),
                        // 5F7GPoqnwnwaudBKYCXYHYQfNBaemhMjVXm3Xgizjne9HUYu
                        hex!["86a8b858dd647620fd2099621650850bb740c6e71bca5b57fd59d4cdc567dd31"].unchecked_into(),
                        // 5FBisezR5yPaUchMxr2s1VJdQ4vUjh76D11NwqXTSXQGG1qy
                        hex!["8a0ed77ec9a0438a5e2dc6558eec6e8e93c13121bd7709f08908e506d74dd60c"].unchecked_into(),
                        // 5F7GPoqnwnwaudBKYCXYHYQfNBaemhMjVXm3Xgizjne9HUYu
                        hex!["86a8b858dd647620fd2099621650850bb740c6e71bca5b57fd59d4cdc567dd31"].unchecked_into(),
                        // 5F7GPoqnwnwaudBKYCXYHYQfNBaemhMjVXm3Xgizjne9HUYu
                        hex!["86a8b858dd647620fd2099621650850bb740c6e71bca5b57fd59d4cdc567dd31"].unchecked_into(),
                    ),
                ],
                // Sudo account
                hex!["f03bb9ee7cba9bf90724ac5bd90fcd9553969448dbd4cd3c88b0ee41a062c515"].into(),
                // Pre-funded accounts
                vec![
                    (hex!["f03bb9ee7cba9bf90724ac5bd90fcd9553969448dbd4cd3c88b0ee41a062c515"].into(), 4999994000000000000), // 499999400
                    // 5FWWuhHThTUSL97FBpXU9EwobbZ6YZqCs8ryRGDvzAqhfzYF	
                    (hex!["98643a2c1477740412cf7b2d7203443626b191523df56ba35ec4a4c5b56b814c"].into(), 1000000000000), // 100
                    // 5Gitx5RkseoZeGcyLmmTA48GZBf2WE3TD4TnrWWgtgm1VJFK	
                    (hex!["ce119a358c2c5e0e1e52301e77c7997dee651ee67a436b03d60252dc5494c03a"].into(), 1000000000000), // 100
                    // 5CAmCtwtLkmBgwNhmkQhmktSh9JRcwKfRv8Ee3m5dJxu1LjD
                    (hex!["049ebece3fe3525306fb16dd804c959684278eec692158320a4c94b11f847e39"].into(), 1000000000000), // 100
                    // 5EgxnoyYB1EvvWUNKvpJFxCG8fsg34f4G2PzkybSiG6DR5Dr
                    (hex!["741f927195ace2d31c4f625cf89556bfa06a764780a1cd7ef333c8a927ab9401"].into(), 1000000000000), // 100
                    // 5CGDWDTdRh28dQ31owjGjuphbhjta95hpRVRpuXdGDe1sZy6
                    (hex!["08c78058ff97641449aa5e0e018d777135324b83a81c704d2da1f6a1ad25df22"].into(), 1000000000000), // 100
                    // 5E1VUev4ym8QBDVE9Wp2oReWuYCfkJJBLnrFxdytqHc7yH5q
                    (hex!["56050eb15aa425e8a502477d7cc0e4bd4b5e55b67a0b340ea27e437ed4595302"].into(), 1000000000000), // 100
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

const STASH: Balance = 500000000000; // 50

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)>,
    root_key: AccountId,
    endowed_accounts: Vec<(AccountId, u128)>,
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
                .map(|k| (k.0.clone(), k.1.clone()))
                .collect(),
        }),
        pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
        pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![] 
		}),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
		// Staking related configs
		pallet_babe: Some(BabeConfig { authorities: vec![] }),
        pallet_democracy: Default::default(),
		//pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
		//pallet_treasury: Some(Default::default()),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
        pallet_collective_Instance1: Some(CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_membership_Instance1: Some(Default::default()),
        pallet_elections_phragmen: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
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
    initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)>,
    root_key: AccountId,
    endowed_accounts: Vec<(AccountId, u128)>,
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
                .map(|k| (k.0.clone(), k.1.clone()))
                .collect(),
        }),
        pallet_sudo: Some(SudoConfig {
			// Assign network admin rights.
			key: root_key,
		}),
        pallet_grandpa: Some(GrandpaConfig {
			authorities: vec![] 
		}),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
		// Staking related configs
		pallet_babe: Some(BabeConfig { authorities: vec![] }),
        pallet_democracy: Default::default(),
		//pallet_authority_discovery: Some(AuthorityDiscoveryConfig { keys: vec![] }),
		pallet_im_online: Some(ImOnlineConfig { keys: vec![] }),
		//pallet_treasury: Some(Default::default()),
		pallet_session: Some(SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		}),
		pallet_staking: Some(StakingConfig {
			validator_count: initial_authorities.len() as u32 * 2,
			minimum_validator_count: initial_authorities.len() as u32,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
        pallet_collective_Instance1: Some(CouncilConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_collective_Instance2: Some(TechnicalCommitteeConfig {
			members: vec![],
			phantom: Default::default(),
		}),
		pallet_membership_Instance1: Some(Default::default()),
        pallet_elections_phragmen: Some(Default::default()),
		pallet_treasury: Some(Default::default()),
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
