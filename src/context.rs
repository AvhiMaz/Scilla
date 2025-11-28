use solana_commitment_config::CommitmentConfig;
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_pubkey::Pubkey;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;

use crate::config::ScillaConfig;

pub struct ScillaContext {
    rpc_client: RpcClient,
    keypair: Keypair,
    pubkey: Pubkey,
}

impl ScillaContext {
    pub fn keypair(&self) -> &Keypair {
        &self.keypair
    }

    pub fn rpc(&self) -> &RpcClient {
        &self.rpc_client
    }

    pub fn pubkey(&self) -> &Pubkey {
        &self.pubkey
    }
}

impl ScillaContext {
    pub fn from_config(config: ScillaConfig) -> anyhow::Result<Self> {
        let rpc_client = RpcClient::new_with_commitment(
            config.rpc_url,
            CommitmentConfig {
                commitment: config.commitment_level,
            },
        );

        use anyhow::anyhow;

        let keypair = Keypair::read_from_file(&config.keypair_path).map_err(|e| {
            anyhow!(
                "Failed to read keypair from {}: {}",
                config.keypair_path.display(),
                e
            )
        })?;

        let pubkey = keypair.pubkey();

        Ok(Self {
            rpc_client,
            keypair,
            pubkey,
        })
    }
}
