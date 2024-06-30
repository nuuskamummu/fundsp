

use portable_atomic::Ordering;

use crate::{target_width::TargetU, TargetAtomicU};

/// Globally unique node ID for a node in a network.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NodeId(TargetU);
static GLOBAL_NODE_ID: TargetAtomicU = TargetAtomicU::new(0);

/// This atomic supplies globally unique IDs.

impl NodeId {
    /// Create a new, globally unique node ID.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        NodeId(GLOBAL_NODE_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub const ID: TargetU = 63;
