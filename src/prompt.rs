// src/prompts.rs
use crate::commands::{
    account::AccountCommand, cluster::ClusterCommand, config::ConfigCommand, stake::StakeCommand,
    vote::VoteCommand, *,
};
use inquire::Select;

pub fn prompt_for_command() -> anyhow::Result<Command> {
    let top_level = Select::new(
        "Choose a command group:",
        vec![
            "Cluster",
            "Stake",
            "Account",
            "Vote",
            "ScillaConfig",
            "Exit",
        ],
    )
    .prompt()?;

    let command = match top_level {
        "Cluster" => Command::Cluster(prompt_cluster()?),
        "Stake" => Command::Stake(prompt_stake()?),
        "Account" => Command::Account(prompt_account()?),
        "Vote" => Command::Vote(prompt_vote()?),
        "ScillaConfig" => Command::ScillaConfig(prompt_config()?),
        "Exit" => Command::Exit,
        _ => unreachable!(),
    };

    Ok(command)
}

fn prompt_cluster() -> anyhow::Result<ClusterCommand> {
    let choice = Select::new(
        "Cluster Command:",
        vec![
            "Epoch Info",
            "Current Slot",
            "Block Height",
            "Block Time",
            "Validators",
            "Cluster Version",
            "Supply Info",
            "Inflation",
        ],
    )
    .prompt()?;

    Ok(match choice {
        "Epoch Info" => ClusterCommand::Epoch,
        "Current Slot" => ClusterCommand::Slot,
        "Block Height" => ClusterCommand::BlockHeight,
        "Block Time" => ClusterCommand::BlockTime,
        "Validators" => ClusterCommand::Validators,
        "Cluster Version" => ClusterCommand::ClusterVersion,
        "Supply Info" => ClusterCommand::Supply,
        "Inflation" => ClusterCommand::Inflation,
        _ => unreachable!(),
    })
}

fn prompt_stake() -> anyhow::Result<StakeCommand> {
    let choice = Select::new(
        "Stake Command:",
        vec![
            "Create Stake Account",
            "Delegate Stake",
            "Deactivate Stake",
            "Withdraw Stake",
            "Merge Stake",
            "Split Stake",
            "Show Stake Account",
            "Stake History",
            "Back",
        ],
    )
    .prompt()?;

    Ok(match choice {
        "Create Stake Account" => StakeCommand::Create,
        "Delegate Stake" => StakeCommand::Delegate,
        "Deactivate Stake" => StakeCommand::Deactivate,
        "Withdraw Stake" => StakeCommand::Withdraw,
        "Merge Stake" => StakeCommand::Merge,
        "Split Stake" => StakeCommand::Split,
        "Show Stake Account" => StakeCommand::Show,
        "Stake History" => StakeCommand::History,
        _ => unreachable!(),
    })
}

fn prompt_account() -> anyhow::Result<AccountCommand> {
    let choice = Select::new(
        "Account Command:",
        vec![
            "Check Balance",
            "Transfer SOL",
            "Request Airdrop",
            "Confirm Transaction",
            "Largest Accounts",
            "Nonce Account Info",
        ],
    )
    .prompt()?;

    Ok(match choice {
        "Check Balance" => AccountCommand::Balance,
        "Transfer SOL" => AccountCommand::Transfer,
        "Request Airdrop" => AccountCommand::Airdrop,
        "Confirm Transaction" => AccountCommand::ConfirmTransaction,
        "Largest Accounts" => AccountCommand::LargestAccounts,
        "Nonce Account Info" => AccountCommand::NonceAccount,
        _ => unreachable!(),
    })
}

fn prompt_vote() -> anyhow::Result<VoteCommand> {
    let choice = Select::new(
        "Vote Command:",
        vec![
            "Create Vote Account",
            "Authorize Voter",
            "Authorize Withdrawer",
            "Show Vote Account",
        ],
    )
    .prompt()?;

    Ok(match choice {
        "Create Vote Account" => VoteCommand::CreateVoteAccount,
        "Authorize Voter" => VoteCommand::AuthorizeVoter,
        "Show Vote Account" => VoteCommand::ShowVoteAccount,
        _ => unreachable!(),
    })
}

fn prompt_config() -> anyhow::Result<ConfigCommand> {
    let choice = Select::new(
        "ScillaConfig Command:",
        vec!["Show ScillaConfig", "Set ScillaConfig", "Edit ScillaConfig"],
    )
    .prompt()?;

    Ok(match choice {
        "Show ScillaConfig" => ConfigCommand::Show,
        "Generate ScillaConfig" => ConfigCommand::Generate,
        "Edit ScillaConfig" => ConfigCommand::Edit,
        _ => unreachable!(),
    })
}
