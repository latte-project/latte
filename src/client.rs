use crate::apis::register::{Lang, Register};
use crate::apis::{get::Get, RequestMessage, ResponseMessage};
use crate::{connection::Connection, func::LatteObject, ObjectRef};
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct Client {
    connection: Connection,
}

pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
    let socket = TcpStream::connect(addr).await?;
    let connection = Connection::new(socket);

    Ok(Client { connection })
}

impl Client {
    pub async fn register(
        &mut self,
        func_name: &String,
        func_body: &String,
        func_lang: Lang,
    ) -> crate::Result<()> {
        self.connection
            .write_request(RequestMessage::Register(Register {
                func_name: func_name.clone(),
                func_lang,
                func_body: func_body.clone(),
            }))
            .await?;
        let resp = self.connection.read_response().await?;
        match resp {
            ResponseMessage::Success => Ok(()),
            ResponseMessage::Fail(msg) => Err(std::io::Error::new(std::io::ErrorKind::Other, msg)),
            _ => {
                let e = format!(
                    "REGISTER RESPONSE ERROR: {}",
                    serde_json::to_string(&resp).unwrap()
                );
                Err(std::io::Error::new(std::io::ErrorKind::Other, e))
            }
        }
    }

    pub async fn invoke(&mut self, func_name: &String, args: &Vec<LatteObject>) -> crate::Result<ObjectRef> {
        
        todo!()
    }

    pub async fn set(
        &mut self,
        object_ref: &ObjectRef,
        latte_object: LatteObject,
    ) -> crate::Result<()> {
        self.connection
            .write_request(RequestMessage::Set(crate::apis::set::Set {
                object_ref: object_ref.clone(),
                latte_object,
            }))
            .await?;
        let resp = self.connection.read_response().await?;
        if let ResponseMessage::Success = resp {
            Ok(())
        } else {
            let e = format!(
                "SET RESPONSE ERROR: {}",
                serde_json::to_string(&resp).unwrap()
            );
            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }

    pub async fn get(&mut self, object_ref: &ObjectRef) -> crate::Result<LatteObject> {
        let req = RequestMessage::Get(Get {
            object_ref: object_ref.clone(),
        });
        self.connection.write_request(req).await?;
        let resp = self.connection.read_response().await?;
        if let ResponseMessage::LatteObject(latte_object) = resp {
            Ok(latte_object)
        } else {
            let e = format!(
                "GET RESPONSE ERROR: {}",
                serde_json::to_string(&resp).unwrap()
            );
            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }
}
