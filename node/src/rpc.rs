//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use pallet_contracts_rpc::{Contracts, ContractsApi};
pub use sc_rpc_api::DenyUnsafe;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};
use sp_transaction_pool::TransactionPool;
use subgame_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Index, Hash};

// EVM
use pallet_ethereum::EthereumStorageSchema;
use fc_rpc::{StorageOverride, SchemaV1Override};
use fc_rpc_core::types::{PendingTransactions, FilterPool};
use jsonrpc_pubsub::manager::SubscriptionManager;
use sc_network::NetworkService;
use std::collections::BTreeMap;
use sc_consensus_manual_seal::rpc::{ManualSeal, ManualSealApi};
use sc_rpc::SubscriptionTaskExecutor;
use sp_runtime::traits::BlakeTwo256;
use sc_client_api::{
	backend::{StorageProvider, Backend, StateBackend, AuxStore},
	client::BlockchainEvents
};

/// Full client dependencies.
pub struct FullDeps<C, P> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// The Node authority flag
	pub is_authority: bool,
	/// Whether to enable dev signer
	pub enable_dev_signer: bool,
	/// Network service
	pub network: Arc<NetworkService<Block, Hash>>,
	/// Ethereum pending transactions.
	pub pending_transactions: PendingTransactions,
	/// EthFilterApi pool.
	pub filter_pool: Option<FilterPool>,
	/// Backend.
	pub backend: Arc<fc_db::Backend<Block>>,
	/// Manual seal command sink
	pub command_sink: Option<futures::channel::mpsc::Sender<sc_consensus_manual_seal::rpc::EngineCommand<Hash>>>,
}

/// Instantiate all full RPC extensions.
pub fn create_full<C, P, BE>(
    deps: FullDeps<C, P>,
	subscription_task_executor: SubscriptionTaskExecutor
) -> jsonrpc_core::IoHandler<sc_rpc::Metadata>
where
    C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
    C: Send + Sync + 'static,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Index>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: BlockBuilder<Block>,
    
    /*** Pallet Contracts ***/
    C::Api: pallet_contracts_rpc::ContractsRuntimeApi<Block, AccountId, Balance, BlockNumber>,
    
    /*** Pallet EVM ***/
    C::Api: fp_rpc::EthereumRuntimeRPCApi<Block>,
    BE: Backend<Block> + 'static,
	BE::State: StateBackend<BlakeTwo256>,
	C: ProvideRuntimeApi<Block> + StorageProvider<Block, BE> + AuxStore,
	C: BlockchainEvents<Block>,
	P: TransactionPool<Block=Block> + 'static,
{
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApi};
    use substrate_frame_rpc_system::{FullSystem, SystemApi};
    use fc_rpc::{
		EthApi, EthApiServer, EthFilterApi, EthFilterApiServer, NetApi, NetApiServer,
		EthPubSubApi, EthPubSubApiServer, Web3Api, Web3ApiServer, EthDevSigner, EthSigner,
		HexEncodedIdProvider,
	};

    let mut io = jsonrpc_core::IoHandler::default();
    let FullDeps {
        client,
        pool,
        deny_unsafe,
		is_authority,
		network,
		pending_transactions,
		filter_pool,
		command_sink,
		backend,
		enable_dev_signer,
    } = deps;

    io.extend_with(SystemApi::to_delegate(FullSystem::new(
        client.clone(),
        pool.clone(),
        deny_unsafe,
    )));

    io.extend_with(TransactionPaymentApi::to_delegate(TransactionPayment::new(
        client.clone(),
    )));

    // Extend this RPC with a custom API by using the following syntax.
    // `YourRpcStruct` should have a reference to a client, which is needed
    // to call into the runtime.
    // `io.extend_with(YourRpcTrait::to_delegate(YourRpcStruct::new(ReferenceToClient, ...)));`

    /*** Pallet Contracts ***/
    io.extend_with(ContractsApi::to_delegate(Contracts::new(client.clone())));
    /*** Pallet Contracts ***/
    
    /*** Pallet EVM ***/
    let mut signers = Vec::new();
	if enable_dev_signer {
		signers.push(Box::new(EthDevSigner::new()) as Box<dyn EthSigner>);
	}
	let mut overrides = BTreeMap::new();
	overrides.insert(
		EthereumStorageSchema::V1,
		Box::new(SchemaV1Override::new(client.clone())) as Box<dyn StorageOverride<_> + Send + Sync>
	);
	io.extend_with(
		EthApiServer::to_delegate(EthApi::new(
			client.clone(),
			pool.clone(),
			subgame_runtime::TransactionConverter,
			network.clone(),
			pending_transactions.clone(),
			signers,
			overrides,
			backend,
			is_authority,
		))
	);

	if let Some(filter_pool) = filter_pool {
		io.extend_with(
			EthFilterApiServer::to_delegate(EthFilterApi::new(
				client.clone(),
				filter_pool.clone(),
				500 as usize, // max stored filters
			))
		);
	}

	io.extend_with(
		NetApiServer::to_delegate(NetApi::new(
			client.clone(),
			network.clone(),
		))
	);

	io.extend_with(
		Web3ApiServer::to_delegate(Web3Api::new(
			client.clone(),
		))
	);

	io.extend_with(
		EthPubSubApiServer::to_delegate(EthPubSubApi::new(
			pool.clone(),
			client.clone(),
			network.clone(),
			SubscriptionManager::<HexEncodedIdProvider>::with_id_provider(
				HexEncodedIdProvider::default(),
				Arc::new(subscription_task_executor)
			),
		))
	);

	match command_sink {
		Some(command_sink) => {
			io.extend_with(
				// We provide the rpc handler with the sending end of the channel to allow the rpc
				// send EngineCommands to the background block authorship task.
				ManualSealApi::to_delegate(ManualSeal::new(command_sink)),
			);
		}
		_ => {}
	}
    /*** Pallet EVM ***/

    io
}
