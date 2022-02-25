use std::collections::HashMap;
use crate::{apis::Command, func::LatteObject};

pub type Db = HashMap<String, LatteObject>;

pub async fn handle_request(req: Command, db: &mut Db) {
    match req {
        Command::Get { cmd, responder } => {
            println!("request get: {}", cmd.object_ref);
            let latte_object = match db.get(&cmd.object_ref) {
                Some(v) => v, 
                None => &LatteObject::Null,
            };
            if let Err(e) = responder.send(latte_object.clone()) {
                println!("responder dropped, object: {}", serde_json::to_string(&e).unwrap());
            }
        }
        Command::Set { cmd, responder } => {
            println!("request set: {} {}", cmd.object_ref, serde_json::to_string(&cmd.latte_object).unwrap());
            db.insert(cmd.object_ref, cmd.latte_object);
            if let Err(_) = responder.send(()) {
                println!("responder dropped");
            }
        }
    }
}
