
#[derive(Clone, Debug)]
pub struct Node {
    pub name: String,
    pub args_in: ArgList,
    pub args_out: ArgList,
    pub locals: Local,
    pub body: EquationList,
}
