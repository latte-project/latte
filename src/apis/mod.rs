use serde::{Serialize, Deserialize};
use tokio::sync::oneshot::Sender;

use crate::{func::LatteObject, ObjectRef};

use self::{get::Get, set::Set};

pub mod get;
pub mod set;

#[derive(Serialize, Deserialize)]
pub enum RequestMessage {
    // Register {
    //     func_spec: FuncSpec, 
    // }, 
    // Invoke {
    //     fname: String,
    //     args: Vec<FuncArgSpec>, 
    // }, 
    Get(get::Get),
    Set(set::Set),
}

#[derive(Serialize, Deserialize)]
pub enum ResponseMessage {
    LatteObject(LatteObject), 
    ObjectRef(ObjectRef),
    Success,
    Fail, 
} 

pub enum Command {
    Get {
        cmd: Get,
        responder: Sender<LatteObject>,
    },
    Set {
        cmd: Set, 
        responder: Sender<()>,
    }
}