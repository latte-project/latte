use serde::{Deserialize, Serialize};

use crate::ObjectRef;

#[derive(Serialize, Deserialize)]
pub(crate) struct Get {
    pub(crate) object_ref: ObjectRef,
}
