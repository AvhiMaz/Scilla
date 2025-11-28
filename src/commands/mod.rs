use crate::{
    commands::{
        account::AccountCommand, cluster::ClusterCommand, config::ConfigCommand,
        stake::StakeCommand, vote::VoteCommand,
    },
    context::ScillaContext,
    error::ScillaResult,
};

pub mod account;
pub mod cluster;
pub mod config;
pub mod stake;
pub mod vote;

#[derive(Debug, Clone)]
pub enum Command {
    Cluster(ClusterCommand),
    Stake(StakeCommand),
    Account(AccountCommand),
    Vote(VoteCommand),
    ScillaConfig(ConfigCommand),
    Exit,
}

impl Command {
    pub async fn process_command(&self, ctx: &ScillaContext) -> ScillaResult<()> {
        match self {
            Command::Cluster(cluster_command) => todo!(),
            Command::Stake(stake_command) => stake_command.process_command(ctx).await?,
            Command::Account(account_command) => account_command.process_command(ctx).await?,
            Command::Vote(vote_command) => todo!(),
            Command::ScillaConfig(config_command) => todo!(),
            Command::Exit => {}
        }
        Ok(())
    }
}
