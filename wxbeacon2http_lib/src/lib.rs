pub mod ble;
pub mod models;
pub mod server;

use std::sync::Arc;
use tokio::sync::RwLock;

use models::EnvDatum;

pub type SharedState = Arc<RwLock<Option<EnvDatum>>>;

pub const WX_COMPANY_ID: u16 = 0x02D5;
pub const WX_LOCAL_NAME: &str = "EP";