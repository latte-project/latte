use std::io::ErrorKind;

use tokio::sync::{mpsc::Sender, oneshot};

use crate::{
    apis::{Command, RequestMessage, ResponseMessage},
    connection::Connection,
};

pub(crate) async fn tcp_reader(
    cmd_tx: Sender<Command>,
    mut connection: Connection,
) -> crate::Result<()> {
    while let Ok(req) = connection.read_request().await {
        match req {
            RequestMessage::Get(get) => {
                let (tx, rx) = oneshot::channel();
                let cmd = Command::Get {
                    cmd: get,
                    responder: tx,
                };
                if let Err(e) = cmd_tx.send(cmd).await {
                    println!("sender dropped, command get, error {}", e);
                }
                if let Ok(lo) = rx.await {
                    let resp = ResponseMessage::LatteObject(lo);
                    connection.write_response(resp).await?
                }
            }
            RequestMessage::Set(set) => {
                let (tx, rx) = oneshot::channel();
                let cmd = Command::Set {
                    cmd: set,
                    responder: tx,
                };
                if let Err(e) = cmd_tx.send(cmd).await {
                    println!("sender dropped, command set, error {}", e);
                }
                if let Ok(()) = rx.await {
                    let resp = ResponseMessage::Success;
                    connection.write_response(resp).await?
                }
            }
            RequestMessage::Register(reg) => {
                let (tx, rx) = oneshot::channel();
                let cmd = Command::Register {
                    cmd: reg,
                    responder: tx,
                };
                if let Err(e) = cmd_tx.send(cmd).await {
                    println!("sender dropped, command register, error {}", e);
                }
                match rx.await {
                    Ok(Ok(_)) => {
                        let resp = ResponseMessage::Success;
                        connection.write_response(resp).await?
                    }
                    Ok(Err(e)) => {
                        let resp = ResponseMessage::Fail(e);
                        connection.write_response(resp).await?
                    }
                    Err(_) => (),
                }
            }
            RequestMessage::Invoke(invoke) => {
                let (tx, rx) = oneshot::channel();
                let cmd = Command::Invoke {
                    cmd: invoke, 
                    responder: tx, 
                };
                if let Err(e) = cmd_tx.send(cmd).await {
                    println!("sender dropped, command invoke, error {}", e);
                }
                match rx.await {
                    Ok(Ok(or)) => {
                        let resp = ResponseMessage::ObjectRef(or);
                        connection.write_response(resp).await?
                    },
                    Ok(Err(e)) => {
                        let resp = ResponseMessage::Fail(e);
                        connection.write_response(resp).await?
                    },
                    Err(_) => ()
                }
            }
        }
    }
    Err(std::io::Error::new(
        ErrorKind::Other,
        "received illegal request",
    ))
}
