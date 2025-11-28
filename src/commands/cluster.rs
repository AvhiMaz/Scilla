/// Commands related to cluster operations
#[derive(Debug, Clone)]
pub enum ClusterCommand {
    Epoch,
    Slot,
    BlockHeight,
    BlockTime,
    Validators,
    Supply,
    Inflation,
    ClusterVersion,
}
