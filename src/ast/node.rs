use crate::ast::{ArgList, EquationList, Local};
use crate::parser::{Pair, Rule};

use anyhow::{anyhow, ensure};

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub args_in: ArgList,
    pub args_out: ArgList,
    pub local: Local,
    pub body: EquationList,
}

impl TryFrom<Pair<'_, Rule>> for Node {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        ensure!(pair.as_rule() == Rule::node, "expected node rule");
        let mut inner = pair.into_inner();
        Ok(Self {
            name: inner
                .next()
                .ok_or(anyhow!("expected next rule"))?
                .as_str()
                .to_string(),
            args_in: ArgList::try_from(inner.next().ok_or(anyhow!("expected next rule"))?)?,
            args_out: ArgList::try_from(inner.next().ok_or(anyhow!("expected next rule"))?)?,
            local: Local::try_from(inner.next().ok_or(anyhow!("expected next rule"))?)?,
            body: EquationList::try_from(inner.next().ok_or(anyhow!("expected next rule"))?)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct List(Vec<Node>);

impl List {
    #[must_use]
    pub fn into_inner(self) -> Vec<Node> {
        self.0
    }
}

impl TryFrom<Pair<'_, Rule>> for List {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        ensure!(pair.as_rule() == Rule::node_list, "expected node list rule");
        let mut list = Vec::new();
        for pair in pair.into_inner() {
            list.push(Node::try_from(pair)?);
        }
        Ok(Self(list))
    }
}
