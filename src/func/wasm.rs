use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct WasmFunc {
    pub(crate) fname: String,
    pub(crate) body: String,
}
