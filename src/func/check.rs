use serde::{Serialize, Deserialize};

use super::{Error, spec::FuncSpec};

#[derive(Serialize, Deserialize)]
pub(crate) struct LatteFunction;

impl FuncSpec {
    pub(crate) fn check(self) -> Result<LatteFunction, Error> {
        // TODO: implementation
        Ok(LatteFunction {})
    }
}