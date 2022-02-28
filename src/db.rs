use crate::{
    apis::Command,
    func::{ftable::FuncTable, LatteObject},
};
use std::collections::HashMap;

pub(crate) type ObjectStore = HashMap<String, LatteObject>;
pub(crate) struct Db {
    os: ObjectStore,
    ftable: FuncTable,
}

impl Db {
    pub(crate) fn new() -> Db {
        Db {
            os: HashMap::new(),
            ftable: HashMap::new(),
        }
    }
}

pub(crate) async fn handle_request(req: Command, db: &mut Db) {
    let os = &mut db.os;
    let ftable = &mut db.ftable;
    let result = match req {
        Command::Get { cmd, responder } => {
            println!("request get: {}", cmd.object_ref);
            let latte_object = match os.get(&cmd.object_ref) {
                Some(v) => v,
                None => &LatteObject::Null,
            };
            responder.send(latte_object.clone()).map_err(|_e| ())
        }
        Command::Set { cmd, responder } => {
            println!(
                "request set: {} {}",
                cmd.object_ref,
                serde_json::to_string(&cmd.latte_object).unwrap()
            );
            os.insert(cmd.object_ref, cmd.latte_object);
            responder.send(()).map_err(|_e| ())
        }
        Command::Register { cmd, responder } => {
            let fname = cmd.func_name.clone();
            println!("request register: {}", fname);
            match cmd.to_spec().check() {
                Ok(f) => {
                    ftable.insert(fname, f);
                    responder.send(Ok(())).map_err(|_e| ())
                }
                Err(e) => responder.send(Err(e)).map_err(|_e| ()),
            }
        }
    };
    let _ = result.map_err(|()| {
        println!("responder dropped");
    });
}
