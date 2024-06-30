use core::sync::atomic::AtomicU32;

use portable_atomic::Ordering;

/// Globally unique node ID for a node in a network.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct NodeId(u32);
static GLOBAL_NODE_ID: AtomicU32 = AtomicU32::new(0);

/// This atomic supplies globally unique IDs.

impl NodeId {
    /// Create a new, globally unique node ID.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        NodeId(GLOBAL_NODE_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub const ID: u32 = 31;
