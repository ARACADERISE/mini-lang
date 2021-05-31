// Importing of the code
use super::starter;
use super::lexer;
use super::parser;
use super::ast;

// Parser
use parser::Parser;
use parser::PFuncs;
use parser::PError;
use parser::PErrorFuncs;

// AST
use ast::Ast;
use ast::AstFuncs;
use ast::AstError;
use ast::AstErrorFuncs;

// Lexer
use lexer::Lexer;
use lexer::LFuncs;
use lexer::Type;
use lexer::LError;
use lexer::LErrorFuncs;

// File Info
use starter::FileInfo;
use starter::FErrors;
use starter::ErrFuncs;
use starter::Funcs;

// Standard Imports
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::env;
use std::io;
use std::fs::File;
use std::io::{BufReader, BufRead};

impl From<io::Error> for FErrors
{
    fn from(err: io::Error) -> FErrors
    {
        FErrors::NoSuchDir(err)
    }
}

impl ErrFuncs for FErrors
{
    fn no_file(file: PathBuf) -> FErrors {
        FErrors::NoSuchFile(file)
    }
    fn file_creation_error(file: PathBuf) -> FErrors
    {
        FErrors::CreateFileErr(file)
    }
}

impl Funcs for FileInfo
{
    fn new() -> Result<FileInfo, io::Error>
    {
        let main_dir = env::current_dir()?;

        Ok(
            Self
            {
                dir: main_dir.to_path_buf(),
                is_valid: main_dir.exists(),
                content: String::new()
            }
        )
    }

    fn append(&mut self, filename: String) -> Result<FileInfo, FErrors>
    {
        self.dir = self.dir.join(filename);

        if self.dir.exists()
        {
            return Ok(self.clone());
        }

        Err(FErrors::NoSuchFile(self.dir.clone()))
    }

    fn read_file(&mut self) -> io::Result<FileInfo>
    {
        let content = BufReader::new(File::open(&self.dir)?);

        for line in content.lines()
        {
            let ln = line?;
            self.content.push_str(&ln);
        }
        Ok(self.clone())
    }
}

impl LFuncs for Lexer
{
    fn new_lexer(info: FileInfo) -> Self
    {
        Self {
            info: info,
            token: Type::Def,
            token_val: String::new(),
            index: 0
        }
    }

    fn advance_with_token(&mut self, token: Type, val: String) -> Type
    {
        self.index += 1;
        self.token = token.clone();

        self.token_val = val;
        return token;
    }

    fn skip_whitespace(&mut self)
    {
        while self.info.content.chars().nth(self.index).unwrap() == ' '
        {
            self.index += 1;
        }
    }

    fn pickup_keyword(&mut self) -> String
    {
        let mut keyword: String = String::new();

        loop
        {
            match self.info.content.chars().nth(self.index)
            {
                Some(x) => {
                    if x != '|' && x != ' ' {
                        keyword.push(x);
                    } else {
                        break;
                    }
                },
                None => break
            }

            self.index += 1;
        }

        return keyword;
    }

    fn pickup_str(&mut self) -> String
    {
        let mut str_val: String = String::new();

        if self.info.content.chars().nth(self.index).unwrap() == '"'
        {
            self.index += 1;
        }

        loop {
            match self.info.content.chars().nth(self.index)
            {
                Some(c) => {
                    if(c == '"')
                    {
                        //self.index += 1;
                        break;
                    }
                    str_val.push(c);
                    self.index += 1;
                },
                None => break
            }
        }

        return str_val;
    }

    fn lex(&mut self) -> Result<Type, LError>
    {
        loop {
            match self.info.content.chars().nth(self.index) {
                Some(' ') => {
                    self.skip_whitespace();
                    continue;
                },
                Some('{') => {
                    return Ok(self.advance_with_token(Type::T_LB, '{'.to_string()));
                }
                Some('}') => {
                    return Ok(self.advance_with_token(Type::T_RB, '}'.to_string()));
                },
                Some('=') => {
                    return Ok(self.advance_with_token(Type::Equals, '='.to_string()));
                },
                Some(';') => {
                    return Ok(self.advance_with_token(Type::Semi, ';'.to_string()));
                }
                Some('"') => {
                    let _str = self.pickup_str();
                    return Ok(self.advance_with_token(Type::Str, _str));
                },
                Some(',') => {
                    return Ok(self.advance_with_token(Type::Comma, ','.to_string()));
                },
                Some('*') => {
                    loop {
                        self.index += 1;

                        match self.info.content.chars().nth(self.index)
                        {
                            Some('*') => {
                                break;
                            }
                            None => break,
                            _ => {}
                        }
                    }
                    self.index += 1
                },
                Some('\t') => {
                    loop {
                        self.index += 1;

                        if self.info.content.chars().nth(self.index) != None
                        {
                            if self.info.content.chars().nth(self.index).unwrap() != ' '
                            {
                                break
                            }
                        }
                    }
                    continue;
                }
                None => {
                    return Ok(self.advance_with_token(Type::EOF, '\0'.to_string()));
                }
                _ => {
                    match self.info.content.chars().nth(self.index).unwrap().is_digit(10) {
                        true => return Ok(self.advance_with_token(Type::NUM, self.info.content.chars().nth(self.index).unwrap().to_string())),
                        false => {
                            if self.info.content.chars().nth(self.index).unwrap().is_alphabetic() == true
                            {
                                let k = self.pickup_keyword();

                                match k.as_str() {
                                    "let" => return Ok(self.advance_with_token(Type::K_LET, k)),
                                    "print" => return Ok(self.advance_with_token(Type::K_PRINT, k)),
                                    _ => return Ok(self.advance_with_token(Type::VarName, k))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl AstErrorFuncs for AstError
{
    fn no_var(varName: String) -> AstError
    {
        AstError::NoSuchVar(varName)
    }

    fn out_of_bounds(index: usize) -> AstError
    {
        AstError::IndexOutOfBounds(index)
    }
}

impl LErrorFuncs for LError
{
    fn token_error(err: Type) -> LError
    {
        LError::TokenErr(err)
    }
}

impl PErrorFuncs for PError
{
    fn unexpected_token(token: Type) -> PError
    {
        PError::UnexpectedToken(token)
    }
}

impl AstFuncs for Ast
{
    fn new_ast() -> Self
    {
        Self {
            sequence: Vec::new(),
            var_names: Vec::new(),
            array_values: BTreeMap::new(),
            var_values: Vec::new(),
            to_print: Vec::new()
        }
    }

    fn i_print(&mut self) -> Result<Ast, AstError>
    {
        println!("{}", self.to_print[0]);
        self.to_print.remove(0);
        Ok(self.clone())
    }

    fn i_variable(&mut self) -> Result<Ast, AstError>
    {
        if self.var_names.len() == self.var_values.len()
        {
            return Ok(self.clone())
        }

        for i in self.var_names.iter()
        {
            if self.array_values.contains_key(i)
            {
                continue;
            }
        }
        Ok(self.clone())
    }

    fn go_through(&mut self) -> Result<Ast, AstError>
    {
        for i in &self.sequence.clone()
        {
            match i {
                Type::VarName => self.i_variable(),
                Type::K_PRINT => self.i_print(),
                _ => return Err(AstError::no_var("".to_string())) // we shouldn't have a problem.
            };
        }
        Ok(self.clone())
    }
}

impl PFuncs for Parser
{
    fn new_parser(lexer: Lexer) -> Self
    {
        Self {
            lex: lexer,
            AST: Ast::new_ast()
        }
    }

    fn get_next_token(&mut self)
    {
        match self.lex.lex()
        {
            Ok(_) => {},
            Err(e) => panic!("{:?}", e)
        }
    }

    fn parse_var_def(&mut self) -> Result<Parser, PError>
    {
        if self.lex.token_val == "let"
        {
            self.get_next_token();

            match self.lex.token
            {
                Type::VarName => {
                    self.AST.var_names.push(self.lex.token_val.clone());
                    self.get_next_token()
                },
                _ => return Err(PError::unexpected_token(self.lex.token.clone()))
            }

            if self.lex.token_val == "="
            {
                self.get_next_token();

                match self.lex.token
                {
                    Type::T_LB => {
                        self.get_next_token();

                        let mut arr: Vec<String> = Vec::new();
                        loop {
                            match self.lex.token
                            {
                                Type::NUM => {
                                    arr.push(self.lex.token_val.clone());
                                    self.get_next_token();
                                },
                                Type::Comma => {
                                    self.get_next_token();
                                },
                                Type::T_RB => {
                                    self.AST.array_values.insert(self.AST.var_names[self.AST.var_names.len() - 1].clone(), arr);
                                    break;
                                }
                                _ => return Err(PError::unexpected_token(self.lex.token.clone()))
                            }
                        }
                        self.get_next_token();

                        match self.lex.token
                        {
                            Type::Semi => {
                                self.AST.sequence.push(Type::VarName);
                                return Ok(self.clone());
                            },
                            Type::EOF => {
                                self.AST.sequence.push(Type::VarName);
                                return Ok(self.clone());
                            },
                            _ => return Err(PError::unexpected_token(self.lex.token.clone()))
                        }
                    },
                    Type::Str => {
                        self.AST.var_values.push(self.lex.token_val.clone());
                        self.get_next_token();

                        match self.lex.token
                        {
                            Type::Semi => return Ok(self.clone()),
                            _ => {}
                        }
                    },
                    _ => return Err(PError::unexpected_token(self.lex.token.clone())),
                }

            }
        }

        self.AST.sequence.push(Type::VarName);
        Ok(self.clone())
    }

    fn parse_print(&mut self) -> Result<Parser, PError>
    {
        self.get_next_token();

        match self.lex.token
        {
            Type::Str => {
                self.AST.to_print.push(self.lex.token_val.clone());
            },
            Type::VarName => {
                self.AST.to_print.push(self.AST.var_values[self.AST.var_values.len() - 1].clone());
            },
            _ => return Err(PError::unexpected_token(self.lex.token.clone()))
        }

        self.AST.sequence.push(Type::K_PRINT);

        self.get_next_token();
        match self.lex.token
        {
            Type::Semi => return Ok(self.clone()),
            _ => {}
        }

        Ok(self.clone())
    }

    fn parse(&mut self, lexer: Lexer) -> Result<Parser, PError>
    {
        self.lex = lexer;

        loop {
            match self.lex.token
            {
                Type::K_LET => {
                    match self.parse_var_def() {
                        Ok(_) => {},
                        Err(t) => return Err(t),
                    }
                },
                Type::K_PRINT => {
                    match self.parse_print() {
                        Ok(_) => {},
                        Err(e) => return Err(e)
                    }
                },
                _ => break
            }
        }

        // Error checking
        self.get_next_token();

        match self.lex.token {
            Type::EOF => {
                return Ok(self.clone());
            },
            _ => return Err(PError::unexpected_token(self.lex.token.clone()))
        }
    }
}
