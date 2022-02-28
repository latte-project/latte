use serde::{Deserialize, Serialize};

use crate::func::{deno::DenoFunc, spec::FuncSpec};

#[derive(Serialize, Deserialize)]
pub(crate) struct Register {
    pub(crate) func_name: String,
    pub(crate) func_lang: Lang,
    pub(crate) func_body: String,
}

#[derive(Serialize, Deserialize)]
pub enum Lang {
    Deno,
}

impl Register {
    pub(crate) fn to_spec(self) -> FuncSpec {
        match self.func_lang {
            Lang::Deno => FuncSpec::Deno(DenoFunc {
                fname: self.func_name,
                body: self.func_body,
            }),
        }
    }
}
