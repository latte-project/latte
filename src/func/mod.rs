pub(crate) mod deno;

use serde::{Serialize, Deserialize};

use self::deno::DenoFunc;

// #[derive(Serialize, Deserialize)]
// pub struct FuncSpec {
//     fname: String, 
//     params: Vec<FuncParamSpec>, 
//     lang: FuncLang, 
//     body: String, 
// }

// #[derive(Serialize, Deserialize)]
// pub enum FuncLang {
//     Rust, 
// }

// #[derive(Serialize, Deserialize)]
// pub struct FuncParamSpec {
//     arg_name: String, 
//     arg_type: LatteObjectType, 
// }

// #[derive(Serialize, Deserialize)]
// pub enum LatteObjectType {
//     Any, 
//     Integer, 
//     String, 
//     Float, 
//     Vector(Box<LatteObjectType>), 
//     // TODO: some other types like map, struct, enum, etc
// }

#[derive(Serialize, Deserialize, Clone)]
pub(crate) enum FuncSpec {
    Deno(DenoFunc)
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum LatteObject {
    Null, 
    Integer(i64), 
    String(String), 
    Float(f64), 
    Vector(Vec<Box<LatteObject>>),
}