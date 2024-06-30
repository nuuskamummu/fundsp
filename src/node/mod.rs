#[cfg(all(not(target_has_atomic = "64"), target_has_atomic = "32"))]
mod node_id32;
#[cfg(all(not(target_has_atomic = "64"), target_has_atomic = "32"))]
pub use node_id32::{NodeId, ID};

#[cfg(target_has_atomic = "64")]
mod node_id64;
#[cfg(target_has_atomic = "64")]
pub use node_id64::{NodeId, ID};
