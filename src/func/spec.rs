use serde::{Serialize, Deserialize};

use super::deno::DenoFunc;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum FuncSpec {
    Deno(DenoFunc)
}
