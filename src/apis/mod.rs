use serde::{Serialize, Deserialize};
use tokio::sync::oneshot::Sender;

use crate::{func::{LatteObject, Error}, ObjectRef};

use self::{get::Get, set::Set, register::Register};

pub mod get;
pub mod set;
pub mod register;

#[derive(Serialize, Deserialize)]
pub(crate) enum RequestMessage {
    Register(register::Register),
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
    Fail(String),
} 

pub(crate) enum Command {
    Get {
        cmd: Get,
        responder: Sender<LatteObject>,
    },
    Set {
        cmd: Set, 
        responder: Sender<()>,
    }, 
    Register {
        cmd: Register, 
        responder: Sender<Result<(), Error>>,
    },
}