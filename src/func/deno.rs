use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct DenoFunc {
    pub(crate) fname: String, 
    pub(crate) body: String, 
}
