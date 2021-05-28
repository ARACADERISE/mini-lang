use super::starter;
use starter::FileInfo;

#[allow(non_camel_case_types)]
#[allow(dead_code)] // get rid of errors, for now
#[derive(Debug, Clone)]
pub enum Type
{
    Def,
    L_SB, // {
    R_SB, // }
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
    pub token: Type
}

pub trait LFuncs
{
    fn new_lexer(info: FileInfo) -> Self;
    fn lex(&mut self) -> Result<Type, LError>;
}

pub trait LErrorFuncs
{
    fn token_error(err: Type) -> LError;
}
