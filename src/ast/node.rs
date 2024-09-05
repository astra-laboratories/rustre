use crate::ast::{ArgList, Equation, EquationList, Locals, Type};
use crate::normalizer::Normalizer;
use crate::parser::{Pair, Rule};
use crate::sequentializer::Sequentializer;
use crate::{next, next_string};

use anyhow::ensure;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub args_in: ArgList,
    pub args_out: ArgList,
    pub locals: Locals,
    pub body: EquationList,
}

impl Node {
    // TODO should this follow the builder pattern?
    // fn(self) -> Self
    pub fn normalize(&mut self) {
        let mut normalizer = Normalizer::new();
        // TODO is this really needed?
        // Prevent local names from being used for intermediates
        //for (name, _) in n.locals.iter() {
        //    intermediates.insert(name.clone(), None);
        //}
        self.body
            .0
            .iter_mut()
            .for_each(|eq| eq.normalize(&mut normalizer));
        normalizer.memory.into_iter().for_each(|(name, expr)| {
            // NOTE: the local name isn't Type::Unit (though we don't use it)
            self.locals.0.insert(name.clone(), Type::Unit);
            self.body.0.push(Equation {
                names: vec![name],
                body: expr,
            });
        });
    }

    // TODO should this follow the builder pattern?
    // fn(self) -> Self
    // also, can this be made nicer?
    pub fn order(&mut self) {
        let mut seq = Sequentializer::build(self).propagate().check();
        let mut ordered_eqs = Vec::<Equation>::new();

        while !seq.dependencies.is_empty() {
            let mut remove = Vec::new();

            for (var, deps) in &seq.dependencies {
                let mut ok = true;
                // Compute: if the dependecies have been met by previously added equations and inputs
                for dep in deps {
                    let is_prev_eq = ordered_eqs.iter().any(|val| val.names.contains(dep));
                    ok = ok && (self.args_in.as_ref().contains_key(dep) || is_prev_eq);
                }
                if ok {
                    // if dependencies satisfied
                    // we put the corresponding equation as the next one to be computed
                    if let Some(eq) = self
                        .body
                        .as_ref()
                        .iter()
                        .find(|eq1| eq1.names.contains(var))
                    {
                        ordered_eqs.push(eq.clone());

                        for key in seq.dependencies.keys() {
                            if eq.names.contains(key) {
                                remove.push(key.clone());
                            }
                        }
                    }
                }
            }

            // removing variables that are also computed by the equation (in tuples)
            // this works because all variables assigned in a tuple all have the same dependecies
            for k in &remove {
                seq.dependencies.remove(k);
            }
        }
        self.body = EquationList(ordered_eqs);
    }
}

impl TryFrom<Pair<'_, Rule>> for Node {
    type Error = anyhow::Error;
    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        ensure!(pair.as_rule() == Rule::node, "expected node rule");
        let mut inner = pair.into_inner();
        Ok(Self {
            name: next_string!(inner),
            args_in: ArgList::try_from(next!(inner))?,
            args_out: ArgList::try_from(next!(inner))?,
            locals: Locals::try_from(next!(inner))?,
            body: EquationList::try_from(next!(inner))?,
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
