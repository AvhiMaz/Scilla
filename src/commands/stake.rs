use crate::{context::ScillaContext, error::ScillaResult};

/// Commands related to staking operations
#[derive(Debug, Clone)]
pub enum StakeCommand {
    Create,
    Delegate,
    Deactivate,
    Withdraw,
    Merge,
    Split,
    Show,
    History,
    Back,
}

impl StakeCommand {
    pub async fn process_command(&self, ctx: &ScillaContext) -> ScillaResult<()> {
        match self {
            StakeCommand::Create => todo!(),
            StakeCommand::Delegate => todo!(),
            StakeCommand::Deactivate => todo!(),
            StakeCommand::Withdraw => todo!(),
            StakeCommand::Merge => todo!(),
            StakeCommand::Split => todo!(),
            StakeCommand::Show => todo!(),
            StakeCommand::History => todo!(),
            StakeCommand::Back => {}
        }
        Ok(())
    }
}
