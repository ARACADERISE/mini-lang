use super::lexer;
use lexer::Type;

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Ast
{
    pub sequence: Vec<Type>,
    pub var_names: Vec<String>,
    pub array_values: BTreeMap<String, Vec<String>>,
    pub var_values: Vec<String>, // convert all values to string
    pub to_print: Vec<String>
}

pub enum AstError
{
    NoSuchVar(String),
    IndexOutOfBounds(usize)
}

pub trait AstFuncs
{
    fn new_ast() -> Self;
    fn i_print(&mut self) -> Result<Ast, AstError>;
    fn i_variable(&mut self) -> Result<Ast, AstError>;
    fn go_through(&mut self) -> Result<Ast, AstError>;
}

pub trait AstErrorFuncs
{
    fn no_var(varName: String) -> AstError;
    fn out_of_bounds(index: usize) -> AstError;
}
