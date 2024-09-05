//! Sequentializer re-orders the equations in the nodes' body
//!
//! This is done in four steps:
//!
//! 1. Generating the direct dependencies for each equation (`find_dep_XXX`)
//!    This is done by adding all the Ident in the left side of each equations to the dependencies
//!    We represent the graph using a `HashMap<String, Vec<String>>`
//!      keys: Varname
//!      values: List of var that the key depends on (children in a Graph)
//!
//! 2. Propagating the dependencies (propagate)
//!    We explore the children of each variable and add their own dependecies to the current Variable
//!    We use a queue of what remains to be explored and keep track of what variable we already visited
//!    so as to not loop endlessly.
//!
//! 3. Checking the satisfiability of the ordering
//!    The only way we could not be able to order the equations is the circular dependency
//!    We can detect those easily by finding the cycles in the graph.
//!    Thanks to propagation we just need to check that no variable depends on itself to be computed.
//!
//! 4. Re-ordering using the dependencies (order)
//!    We construct the Node's body incrementally
//!    Each loop turn we check wether or not all the dependecies of each equation has been met.
//!    If it has we can append this equation to the body
//!    We repeat this until all the equations are placed in the body.

use crate::ast::Node;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, Default)]
pub struct Sequentializer {
    pub dependencies: HashMap<String, Vec<String>>,
}

impl Sequentializer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a dependency graph from a node.
    ///
    /// # Panics
    ///
    /// Panics if a dependency is defined by multiple equations in the same node.
    #[must_use]
    pub fn build(node: &Node) -> Self {
        // TODO can this be nicer?
        let mut dependencies = HashMap::new();
        for eq in node.body.as_ref() {
            let deps = eq.dependencies();
            for name in &eq.names {
                assert!(
                    !dependencies.contains_key(name),
                    "Multiple equations define `{}` in node `{}`",
                    name,
                    node.name
                );
                dependencies.insert(name.clone(), deps.clone());
            }
        }
        Self { dependencies }
    }

    #[must_use]
    pub fn propagate(self) -> Self {
        // TODO can this be nicer?
        let dependencies = self
            .dependencies
            .clone()
            .into_iter()
            .map(|(key, value)| {
                let mut queue = VecDeque::from(value);
                let mut agg = HashSet::<String>::new();
                while let Some(first) = queue.pop_front() {
                    if let Some(vals) = self.dependencies.get(&first) {
                        for val in vals {
                            if !(agg.contains(val) || queue.contains(val)) {
                                queue.push_back(val.clone());
                            }
                        }
                    }
                    if !agg.contains(&first) {
                        agg.insert(first);
                    }
                }
                (key, agg.into_iter().collect())
            })
            .collect();
        Self { dependencies }
    }

    /// Checks if the dependency graph is free of circular dependencies.
    ///
    /// # Panics
    ///
    /// Panics if the dependency graph contains circular dependencies.
    #[must_use]
    pub fn check(self) -> Self {
        assert!(
            !self
                .dependencies
                .iter()
                .any(|(key, value)| value.contains(key)),
            "circular dependency detected"
        );
        self
    }
}
