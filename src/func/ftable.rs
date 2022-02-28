use std::collections::HashMap;

use super::check::LatteFunction;

type FunctionRef = String;
pub(crate) type FuncTable = HashMap<FunctionRef, LatteFunction>;
