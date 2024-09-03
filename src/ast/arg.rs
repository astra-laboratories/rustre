use crate::ast::Type;
use crate::next;
use crate::parser::{Pair, Rule};

use anyhow::ensure;

use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Arg {
    pub typ: Type,
    pub names: Vec<String>,
}

impl TryFrom<Pair<'_, Rule>> for Arg {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        ensure!(pair.as_rule() == Rule::arg, "expected arg rule");
        let mut inner = pair.into_inner();
        let names = next!(inner)
            .into_inner()
            .map(|p| p.as_str().to_string())
            .collect();
        let typ = Type::try_from(next!(inner))?;
        Ok(Self { typ, names })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct List(HashMap<String, Type>);

impl List {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn map(&self) -> &HashMap<String, Type> {
        &self.0
    }

    pub fn insert_arg(&mut self, arg: Arg) {
        arg.names.into_iter().for_each(|name| {
            self.0.insert(name, arg.typ.clone());
        });
    }
}

impl TryFrom<Pair<'_, Rule>> for List {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        ensure!(pair.as_rule() == Rule::arg_list, "expected arg list rule");
        let mut list = Self::new();
        for arg_iden in pair.into_inner() {
            let arg = Arg::try_from(arg_iden)?;
            list.insert_arg(arg);
        }
        Ok(list)
    }
}

// TODO more descriptive name?
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Local(pub HashMap<String, Type>);

impl Local {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl TryFrom<Pair<'_, Rule>> for Local {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        ensure!(pair.as_rule() == Rule::local, "expected local rule");
        let local = if let Some(inner) = pair.into_inner().next() {
            Local::from(List::try_from(inner)?)
        } else {
            Local::new()
        };
        Ok(local)
    }
}

impl From<List> for Local {
    fn from(list: List) -> Self {
        Self(list.0)
    }
}
