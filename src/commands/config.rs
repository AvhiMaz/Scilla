/// Commands related to configuration like RPC_URL , KEYAPAIR_PATH etc
#[derive(Debug, Clone)]
pub enum ConfigCommand {
    Show,
    Generate,
    Edit,
}
