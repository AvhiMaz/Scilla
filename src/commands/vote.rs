/// Commands related to validator/vote account operations
#[derive(Debug, Clone)]
pub enum VoteCommand {
    CreateVoteAccount,
    AuthorizeVoter,
    WithdrawFromVoteAccount,
    ShowVoteAccount,
}
