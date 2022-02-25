use tokio::net::{ToSocketAddrs, TcpStream};
use crate::{connection::Connection, func::LatteObject, apis::{ResponseMessage, get::Get}, ObjectRef};
// use crate::func::FuncSpec;
// use crate::apis::FuncArgSpec;

pub struct Client {
    connection: Connection,
}

pub async fn connect<T: ToSocketAddrs>(addr: T) -> crate::Result<Client> {
    let socket = TcpStream::connect(addr).await?;
    let connection = Connection::new(socket);

    Ok(Client { connection })
}

impl Client {
    // pub async fn register(&mut self, func_spec: FuncSpec) -> crate::Result<()> {
    //     self.connection.write_request(crate::apis::RequestMessage::Register { func_spec }).await
    // }

    // pub async fn invoke(&mut self, fname: &String, args: &Vec<FuncArgSpec>) -> crate::Result<ObjectRef> {
    //     self.connection.write_request(crate::apis::RequestMessage::Invoke {
    //         fname: fname.clone(),
    //         args: args.clone(),
    //     }).await?;
    //     let resp = self.connection.read_response().await?;
    //     if let ResponseMessage::ObjectRef(object_ref) = resp {
    //         Ok(object_ref)
    //     } else {
    //         panic!()
    //     }
    // }

    pub async fn set(&mut self, object_ref: &ObjectRef, latte_object: LatteObject) -> crate::Result<()> {
        self.connection.write_request(crate::apis::RequestMessage::Set(crate::apis::set::Set {
            object_ref: object_ref.clone(),
            latte_object,
        })).await?;
        let resp = self.connection.read_response().await?;
        if let ResponseMessage::Success = resp {
            Ok(())
        } else {
            let e = format!("SET RESPONSE ERROR: {}", serde_json::to_string(&resp).unwrap());
            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }

    pub async fn get(&mut self, object_ref: &ObjectRef) -> crate::Result<LatteObject> {
        let req = crate::apis::RequestMessage::Get(Get { object_ref: object_ref.clone() });
        self.connection.write_request(req).await?;
        let resp = self.connection.read_response().await?;
        if let ResponseMessage::LatteObject(latte_object) = resp {
            Ok(latte_object)
        } else {
            let e = format!("GET RESPONSE ERROR: {}", serde_json::to_string(&resp).unwrap());
            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }
}