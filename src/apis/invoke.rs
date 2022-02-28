use serde::{Deserialize, Serialize};

use crate::func::LatteObject;

#[derive(Serialize, Deserialize)]
pub(crate) struct Invoke {
    pub(crate) fname: String, 
    pub(crate) args: Vec<LatteObject>, 
}
