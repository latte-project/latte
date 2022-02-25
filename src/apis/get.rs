use serde::{Deserialize, Serialize};

use crate::{func::LatteObject, ObjectRef};

#[derive(Serialize, Deserialize, Clone)]
pub struct FuncArgSpec(LatteObject);

#[derive(Serialize, Deserialize)]
pub struct Get {
    pub(crate) object_ref: ObjectRef,
}
