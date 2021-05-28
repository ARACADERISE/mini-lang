use super::lexer;

use lexer::Lexer;
use lexer::Type;

#[derive(Debug)]
pub enum PError
{
    UnexpectedToken(Type)
}

#[derive(Debug, Clone)]
pub struct Parser
{
    pub lex: Lexer
}

pub trait PFuncs
{
    fn new_parser(lexer: Lexer) -> Self;
    fn parse_var_def(&mut self) -> Result<Parser, PError>;
    fn parse_print(&mut self) -> Result<Parser, PError>;
    fn get_next_token(&mut self);
    fn parse(&mut self, lexer: Lexer) -> Result<Parser, PError>;
}

pub trait PErrorFuncs
{
    fn unexpected_token(token: Type) -> PError;
}
