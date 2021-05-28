use super::starter;
use starter::FileInfo;

#[allow(non_camel_case_types)]
#[allow(dead_code)] // get rid of errors, for now
#[derive(Debug, Clone)]
pub enum Type
{
    Def,
    Tab,
    Comma,
    Equals,
    K_LET,
    VarName,
    T_LB, // {
    T_RB, // }
    NUM,  // Number,
    EOF,  // \0
}

#[derive(Debug)]
pub enum LError
{
    TokenErr(Type)
}

#[derive(Debug, Clone)]
pub struct Lexer
{
    pub info: FileInfo,
    pub token: Type,
    pub index: usize,
    pub token_val: String
}

pub trait LFuncs
{
    fn new_lexer(info: FileInfo) -> Self;
    fn advance_with_token(&mut self, token: Type, val: String) -> Type;
    fn pickup_keyword(&mut self) -> String;
    fn skip_whitespace(&mut self);
    fn lex(&mut self) -> Result<Type, LError>;
}

pub trait LErrorFuncs
{
    fn token_error(err: Type) -> LError;
}
