use console::style;

use crate::{context::ScillaContext, error::ScillaResult, ui::show_spinner};

/// Commands related to wallet or account management
#[derive(Debug, Clone)]
pub enum AccountCommand {
    Balance,
    Transfer,
    Airdrop,
    ConfirmTransaction,
    LargestAccounts,
    NonceAccount,
}

impl AccountCommand {
    pub fn description(&self) -> &'static str {
        match self {
            AccountCommand::Balance => "Get wallet balance",
            AccountCommand::Transfer => "Transfer SOL to another address",
            AccountCommand::Airdrop => "Request SOL from faucet",
            AccountCommand::ConfirmTransaction => "Confirm a pending transaction",
            AccountCommand::LargestAccounts => "Fetch cluster’s largest accounts",
            AccountCommand::NonceAccount => "Inspect or manage nonce accounts",
        }
    }
}

impl AccountCommand {
    pub async fn process_command(&self, ctx: &ScillaContext) -> ScillaResult<()> {
        let task = match self {
            AccountCommand::Balance => todo!(),
            AccountCommand::Transfer => todo!(),
            AccountCommand::Airdrop => request_sol_airdrop(&ctx),
            AccountCommand::ConfirmTransaction => todo!(),
            AccountCommand::LargestAccounts => todo!(),
            AccountCommand::NonceAccount => todo!(),
        };

        show_spinner(self.description(), task).await?;
        Ok(())
    }
}

async fn request_sol_airdrop(ctx: &ScillaContext) -> ScillaResult<()> {
    let sig = ctx.rpc().request_airdrop(ctx.pubkey(), 1).await;
    match sig {
        Ok(signature) => {
            println!(
                "{} {}",
                style("Airdrop requested successfully!").green().bold(),
                style(format!("Signature: {}", signature)).cyan()
            );
        }
        Err(err) => {
            eprintln!(
                "{} {}",
                style("Airdrop failed:").red().bold(),
                style(err).red()
            );
        }
    }

    Ok(())
}
