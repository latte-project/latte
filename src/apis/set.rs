use crate::{ObjectRef, func::LatteObject};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Set {
    pub(crate) object_ref: ObjectRef, 
    pub(crate) latte_object: LatteObject,
}
