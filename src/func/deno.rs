use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct DenoFunc {
    fname: String, 
    body: String, 
}