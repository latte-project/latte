use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::Sender;

use crate::{
    func::{Error, LatteObject},
    ObjectRef,
};

use self::{get::Get, register::Register, set::Set, invoke::Invoke};

pub mod get;
pub mod register;
pub mod set;
pub mod invoke;

#[derive(Serialize, Deserialize)]
pub(crate) enum RequestMessage {
    Register(register::Register),
    Invoke(invoke::Invoke),
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
    Invoke {
        cmd: Invoke, 
        responder: Sender<Result<ObjectRef, Error>>,
    }
}
