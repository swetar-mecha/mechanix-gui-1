pub mod network_sync;
pub mod bluetooth_sync;

pub use network_sync::{sync_network_status, sync_network_strength, sync_connected_network};
pub use bluetooth_sync::{sync_bluetooth_status, sync_bluetooth_connected_status};