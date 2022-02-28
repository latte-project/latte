use serde::{Deserialize, Serialize};

use super::deno::DenoFunc;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum FuncSpec {
    Deno(DenoFunc),
}
