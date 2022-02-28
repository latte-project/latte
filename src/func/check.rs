use serde::{Deserialize, Serialize};

use super::{spec::FuncSpec, Error};

#[derive(Serialize, Deserialize)]
pub(crate) struct LatteFunction;

impl FuncSpec {
    pub(crate) fn check(self) -> Result<LatteFunction, Error> {
        // TODO: implementation
        Ok(LatteFunction {})
    }
}
