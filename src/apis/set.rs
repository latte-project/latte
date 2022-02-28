use crate::{func::LatteObject, ObjectRef};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct Set {
    pub(crate) object_ref: ObjectRef,
    pub(crate) latte_object: LatteObject,
}
