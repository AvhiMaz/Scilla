use crate::{
    config::ScillaConfig, context::ScillaContext, error::ScillaResult, prompt::prompt_for_command,
};
use console::style;

pub mod commands;
pub mod config;
pub mod context;
pub mod error;
pub mod prompt;
pub mod ui;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> ScillaResult<()> {
    let config = match ScillaConfig::load() {
        Ok(config) => config,
        Err(e) => return Err(e.into()),
    };

    let ctx = ScillaContext::from_config(config)?;

    println!(
        "{}",
        style("⚡ Scilla — Welcome to The Matrix").bold().cyan()
    );

    loop {
        let command = prompt_for_command()?;

        let _res = command.process_command(&ctx).await;

        break;
    }

    println!("{}", style("Goodbye 👋").dim());

    Ok(())
}
