use std::collections::HashMap;
use crate::{apis::Command, func::{LatteObject, ftable::FuncTable}};

pub(crate) type ObjectStore = HashMap<String, LatteObject>;
pub(crate) struct Db {
    os: ObjectStore, 
    ftable: FuncTable,
}

impl Db {
    pub(crate) fn new() -> Db {
        Db { os: HashMap::new(), ftable: HashMap::new() }
    }
}

pub(crate) async fn handle_request(req: Command, db: &mut Db) {
    let os = &mut db.os;
    let ftable = &mut db.ftable;
    match req {
        Command::Get { cmd, responder } => {
            println!("request get: {}", cmd.object_ref);
            let latte_object = match os.get(&cmd.object_ref) {
                Some(v) => v, 
                None => &LatteObject::Null,
            };
            if let Err(e) = responder.send(latte_object.clone()) {
                println!("responder dropped, object: {}", serde_json::to_string(&e).unwrap());
            }
        }
        Command::Set { cmd, responder } => {
            println!("request set: {} {}", cmd.object_ref, serde_json::to_string(&cmd.latte_object).unwrap());
            os.insert(cmd.object_ref, cmd.latte_object);
            if let Err(_) = responder.send(()) {
                println!("responder dropped");
            }
        }
        Command::Register { cmd, responder } => {
            let fname = cmd.func_name.clone();
            println!("request register: {}", fname);
            match cmd.to_spec().check() {
                Ok(f) => {
                    ftable.insert(fname, f);
                    if let Err(_) = responder.send(Ok(())) {
                        println!("responder dropped");
                    }
                }
                Err(e) => {
                    if let Err(_) = responder.send(Err(e)) {
                        println!("responder dropped")
                    }
                }
            }
        }
    }
}
