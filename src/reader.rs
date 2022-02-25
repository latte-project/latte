use std::io::ErrorKind;

use tokio::sync::{mpsc::Sender, oneshot};

use crate::{apis::{RequestMessage, Command, ResponseMessage}, connection::Connection};

pub async fn tcp_reader(cmd_tx: Sender<Command>, mut connection: Connection) -> crate::Result<()> {
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
            },
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
            },
        }
    }
    Err(std::io::Error::new(ErrorKind::Other, "received illegal request"))
}