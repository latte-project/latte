pub(crate) mod deno;
pub(crate) mod ftable;
pub(crate) mod check;
pub(crate) mod spec;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum LatteObject {
    Null, 
    Integer(i64), 
    String(String), 
    Float(f64), 
    Vector(Vec<Box<LatteObject>>),
}

pub(crate) type Error = String;