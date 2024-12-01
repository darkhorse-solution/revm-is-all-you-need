use std::collections::BTreeSet;
use std::sync::Arc;
use ethers_core::types::BlockNumber;
use ethers_providers::{Middleware, Provider, Ws};
use revm::{
    db::{CacheDB, EmptyDB, InMemoryDB},
    primitives::{

    },
    EVM
};

use foundry_evm_mini::evm::executor::{
    fork::{BlockchainDb, BlockchainDbMeta, SharedBackend},
    inspector::{get_precompiles_for, AccessListTracer},
};

pub async fn evm_main() {
    let wss_url = "wss://eth-mainnet.g.alchemy.com/v2/ainc64D6aQZ6i7QwDkYqUkSVj754iwGz";
    let ws = Ws::connect(wss_url).await.unwrap();
    let provider = Arc::new(Provider::new(ws));

    let block = provider
        .get_block(BlockNumber::Latest)
        .await
        .unwrap()
        .unwrap();

    let shared_backend = SharedBackend::spawn_backend_thread(
        provider.clone(),
        BlockchainDb::new(
            BlockchainDbMeta {
                cfg_env: Default::default(),
                block_env: Default::default(),
                hosts: BTreeSet::from(["".to_string()]),
            },
            None,
        ),
        Some(block.number.unwrap().into()),
    );
    let db = CacheDB::new(shared_backend);

    let mut evm = EVM::new();
    evm.database(db);
}