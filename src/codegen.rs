//! Rust code generation.
//!
//! Generates Rust code from a normalized and scheduled AST.
//!
//! This also compiles `fby` operators. This is the most interesting step.
//!
//! First, each node is assigned a memory if needed. This is done by the `get_node_mem` function,
//! which returns a `NodeMemory`. A node memory will contain two kind of objects:
//!
//! - One field per `fby` operator, which is initialized with the constant on the left of the
//!   operator
//! - One field per function call, which contains the memory of the callee
//!
//! Each node will translate to a Rust function and will take a mutable reference to its memory as
//! the first parameter.
//!
//! Once each node has a memory, we can start generating code.
//!
//! When generating a node's code, we replace `fby` operators to an access to the memory field. We
//! also add a function footer to update `fby` memory fields to their next value (expression on the
//! right of `fby`).
//!
//! When calling another node, we borrow a mutable reference to the call memory field. This is
//! possible because we have a mutable reference to our own memory. We provide this "sub-reference"
//! to the callee.

use crate::ast::{Const, Expr, Node};

use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Memory {
    pub name: String,
    /// Name and type of each field
    pub fields: HashMap<String, String>,
    pub init_values: HashMap<String, Vec<Const>>,
    pub next_values: HashMap<String, Expr>,
}

impl Memory {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn build(node: &Node, mems: &HashMap<String, Memory>) -> Self {
        let mut fields = HashMap::new(); // Memory fields (both for function calls and `fby`)
        let mut init_values = HashMap::new(); // Initialization values for each field (only for `fby`)
        let mut next_values = HashMap::new(); // Next values for each field (only for `fby`)
        for eq in node.body.as_ref() {
            let dest = eq.names.join("_");
            match &eq.body {
                Expr::Call { name, args: _ } => {
                    if let Some(call_mem) = mems.get(name) {
                        fields.insert(dest, call_mem.name.clone());
                    }
                }
                Expr::Fby(lhs, rhs) => {
                    let typ = match lhs.as_ref() {
                        Expr::Const(c) => c.as_type(),
                        _ => unreachable!(),
                    };
                    init_values.insert(dest.clone(), lhs.clone());
                    next_values.insert(dest.clone(), rhs.clone());
                    fields.insert(dest.clone(), typ.to_string());
                }
                _ => {}
            }
        }

        Self {
            name: format!("Mem{}", capitalize(&n.name)),
            fields: fields,
            init_values: init_values,
            next_values: next_values,
        }
    }
}

/*
use std::collections::HashMap;
use std::io::{Write, Result};
use crate::typer::type_of_const;

fn format_expr(w: &mut Write, e: &Expr, dest: &[String], mems: &HashMap<String, NodeMemory>) -> Result<()> {
    match e {
        Expr::Call{name, args} => {
            write!(w, "{}(", name)?;
            let mut first = true;
            if let Some(_) = mems.get(name) {
                if dest.is_empty() {
                    // Used in main()
                    write!(w, "&mut mem")?;
                } else {
                    write!(w, "&mut mem.{}", dest.join("_"))?;
                }
                first = false;
            }
            for arg in args {
                if !first {
                    write!(w, ", ")?;
                }
                first = false;
                format_bexpr(w, arg)?;
            }
            write!(w, ")")
        },
        Expr::Fby(_, _) => {
            write!(w, "mem.{}", dest.join("_"))
        },
        Expr::Bexpr(bexpr) => format_bexpr(w, bexpr),
    }
}

fn format_equation(w: &mut Write, eq: &Equation, mems: &HashMap<String, NodeMemory>) -> Result<()> {
    write!(w, "\tlet ")?;
    if eq.names.len() != 1 {
        write!(w, "(")?;
    }
    let mut first = true;
    for name in &eq.names {
        if !first {
            write!(w, ", ")?;
        }
        first = false;
        write!(w, "{}", name)?;
    }
    if eq.names.len() != 1 {
        write!(w, ")")?;
    }
    write!(w, " = ")?;
    format_expr(w, &eq.body, &eq.names, mems)?;
    write!(w, ";\n")
}

fn format_arg_list(w: &mut Write, args: &HashMap<String, Type>, with_name: bool, with_typ: bool) -> Result<()> {
    let mut first = true;
    for (name, typ) in args {
        if !first {
            write!(w, ", ")?;
        }
        first = false;
        if with_name {
            write!(w, "{}", name)?;
        }
        if with_name && with_typ {
            write!(w, ": ")?;
        }
        if with_typ {
            write!(w, "{}", get_type(typ))?;
        }
    }
    Ok(())
}

fn format_struct(w: &mut Write, name: &str, fields: &HashMap<String, String>, init_values: &HashMap<String, Vec<Const>>) -> Result<()> {
    write!(w, "#[derive(Debug)]\n")?;
    write!(w, "struct {} {{\n", name)?;
    for (k, t) in fields {
        write!(w, "\t{}: {},\n", k, t)?;
    }
    write!(w, "}}\n\n")?;

    write!(w, "impl Default for {} {{\n", name)?;
    write!(w, "\tfn default() -> Self {{\n")?;
    write!(w, "\t\tSelf {{\n")?;
    for (k, _) in fields {
        write!(w, "\t\t\t{}: ", k)?;
        match init_values.get(k) {
            Some(consts) => {
                if consts.len() == 1 {
                    format_const(w, &consts[0])?;
                } else {
                    write!(w, "(")?;
                    let mut first = true;
                    for c in consts {
                        if !first {
                            write!(w, ", ")?;
                        }
                        first = false;
                        format_const(w, c)?;
                    }
                    write!(w, ")")?;
                }
            },
            None => write!(w, "Default::default()")?,
        }
        write!(w, ",\n")?;
    }
    write!(w, "\t\t}}\n")?;
    write!(w, "\t}}\n")?;
    write!(w, "}}\n\n")
}

fn bexpr_from_vec(v: Vec<Bexpr>) -> Bexpr {
    if v.is_empty() {
        Bexpr::Atom(Atom::Const(Const::Unit))
    } else if v.len() == 1 {
        v.into_iter().nth(0).unwrap()
    } else {
        Bexpr::Tuple(v)
    }
}

struct NodeMemory {
    name: String,
    /// Name and type of each field
    fields: HashMap<String, String>,
    init_values: HashMap<String, Vec<Const>>,
    next_values: HashMap<String, Bexpr>,
}

fn get_node_mem(n: &Node, mems: &HashMap<String, NodeMemory>) -> Option<NodeMemory> {
    let mut fields = HashMap::new(); // Memory fields (both for function calls and `fby`)
    let mut init_values = HashMap::new(); // Initialization values for each field (only for `fby`)
    let mut next_values = HashMap::new(); // Next values for each field (only for `fby`)
    for eq in &n.body {
        let dest = eq.names.join("_");
        match &eq.body {
            Expr::Call{name, args: _} => {
                if let Some(call_mem) = mems.get(name) {
                    fields.insert(dest, call_mem.name.clone());
                }
            },
            Expr::Fby(init, next) => {
                let init: Vec<Const> = init.iter().map(|atom| match atom {
                    Atom::Const(c) => c.clone(),
                    _ => unreachable!(),
                }).collect();
                let next = bexpr_from_vec(next.clone());
                let t = match init.len() {
                    0 => Type::Unit,
                    1 => type_of_const(&init[0]),
                    _ => Type::Tuple(init.iter().map(type_of_const).collect()),
                };
                init_values.insert(dest.clone(), init.clone());
                next_values.insert(dest.clone(), next.clone());
                fields.insert(dest.clone(), get_type(&t).to_string());
            },
            _ => {},
        }
    }

    if fields.len() == 0 {
        None
    } else {
        Some(NodeMemory{
            name: format!("Mem{}", capitalize(&n.name)),
            fields: fields,
            init_values: init_values,
            next_values: next_values,
        })
    }
}

fn format_node(w: &mut Write, n: &Node, mems: &HashMap<String, NodeMemory>) -> Result<()> {
    let mem = mems.get(&n.name);
    if let Some(mem) = mem {
        format_struct(w, &mem.name, &mem.fields, &mem.init_values)?;
    }

    write!(w, "fn {}(", &n.name)?;
    if let Some(mem) = mem {
        write!(w, "mem: &mut {}", &mem.name)?;
        if !n.args_in.is_empty() {
            write!(w, ", ")?;
        }
    }
    format_arg_list(w, &n.args_in, true, true)?;
    write!(w, ") -> ")?;
    if n.args_out.len() > 1 {
        write!(w, "(")?;
    }
    format_arg_list(w, &n.args_out, false, true)?;
    if n.args_out.len() > 1 {
        write!(w, ")")?;
    }
    write!(w, " {{\n")?;
    for eq in &n.body {
        format_equation(w, eq, mems)?;
    }

    if let Some(mem) = mem {
        for (k, v) in &mem.next_values {
            write!(w, "\tmem.{} = ", k)?;
            format_bexpr(w, v)?;
            write!(w, ";\n")?;
        }
    }

    write!(w, "\treturn ")?;
    if n.args_out.len() > 1 {
        write!(w, "(")?;
    }
    format_arg_list(w, &n.args_out, true, false)?;
    if n.args_out.len() > 1 {
        write!(w, ")")?;
    }
    write!(w, ";\n")?;
    write!(w, "}}\n\n")
}

pub fn format(w: &mut Write, f: &[Node]) -> Result<()> {
    // Builtin functions
    write!(w, "#[allow(dead_code)]\n");
    write!(w, "fn print(s: &str) {{\n")?;
    write!(w, "\tprintln!(\"{{}}\", s);\n")?;
    write!(w, "}}\n\n")?;

    write!(w, "#[allow(dead_code)]\n");
    write!(w, "fn not(b: bool) -> bool {{\n")?;
    write!(w, "\treturn !b;\n")?;
    write!(w, "}}\n\n")?;

    write!(w, "#[allow(dead_code)]\n");
    write!(w, "fn cos(f: f32) -> f32 {{\n")?;
    write!(w, "\treturn f.cos();\n")?;
    write!(w, "}}\n\n")?;

    write!(w, "#[allow(dead_code)]\n");
    write!(w, "fn sin(f: f32) -> f32 {{\n")?;
    write!(w, "\treturn f.sin();\n")?;
    write!(w, "}}\n\n")?;

    write!(w, "#[allow(dead_code)]\n");
    write!(w, "fn float_of_int(i: i32) -> f32 {{\n")?;
    write!(w, "\treturn i as f32;\n")?;
    write!(w, "}}\n\n")?;

    write!(w, "#[allow(dead_code)]\n");
    write!(w, "fn int_of_float(f: f32) -> i32 {{\n")?;
    write!(w, "\treturn f as i32;\n")?;
    write!(w, "}}\n\n")?;

    // Create one memory per node, if needed
    let mut mems = HashMap::new();
    for n in f {
        if let Some(mem) = get_node_mem(n, &mems) {
            mems.insert(n.name.clone(), mem);
        }
    }

    // Generate code for each node
    for n in f {
        format_node(w, n, &mems)?;
    }

    // Call the last node in main()
    write!(w, "fn main() {{\n")?;
    if let Some(n) = f.last() {
        let num_calls = 10;
        write!(w, "\teprintln!(\"We will call node `{}` {} times.\");\n", &n.name, num_calls)?;

        // Ask input arguments
        for (name, typ) in &n.args_in {
            if let Type::Unit = typ {
                write!(w, "\tlet {} = ();\n", name)?;
                continue
            }

            write!(w, "\teprint!(\"{}: \");\n", name)?;
            write!(w, "\tlet mut {}_str = String::new();\n", name)?;
            write!(w, "\tstd::io::stdin().read_line(&mut {}_str).unwrap();\n", name)?;
            match typ {
                Type::String => write!(w, "\tlet {} = {}_str;\n", name, name)?,
                _ => write!(w, "\tlet {} = {}_str.trim().parse::<{}>().unwrap();\n", name, name, get_type(typ))?,
            }
            write!(w, "\n")?;
        }

        let argv = n.args_in.iter().map(|(name, _)| {
            Bexpr::Atom(Atom::Ident(name.clone()))
        }).collect();
        let call = Expr::Call{
            name: n.name.clone(),
            args: argv,
        };

        // Initialize the callee memory
        if let Some(call_mem) = mems.get(&n.name) {
            write!(w, "\tlet mut mem: {} = Default::default();\n", &call_mem.name)?;
        }

        // Call the node in a loop
        write!(w, "\tfor _ in 0..{} {{\n", num_calls)?;

        write!(w, "\t\tlet v = ")?;
        format_expr(w, &call, &vec![], &mems)?;
        write!(w, ";\n")?;

        write!(w, "\t\teprintln!(\"{{:?}}\", &v);\n")?;

        write!(w, "\t}}\n")?;
    }
    write!(w, "}}\n")
}
*/
