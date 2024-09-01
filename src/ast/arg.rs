use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Arg {
    pub typ: Type,
    pub names: Vec<String>,
}

pub struct ArgList(HashMap<String, Type>);

impl From<Arg> for ArgList {
    fn from(arg: Arg) -> Self {
        let map = arg
            .names
            .into_iter()
            .map(|name| (name, arg.typ.clone()))
            .collect();
        Self(map)
    }
}
