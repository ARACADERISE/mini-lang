// Importing of the code
use super::starter;
use super::lexer;
use super::parser;

// Parser
use parser::Parser;
use parser::PFuncs;
use parser::PError;
use parser::PErrorFuncs;

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
        self.index += 1;
    }

    fn pickup_keyword(&mut self) -> String
    {
        let mut keyword: String = String::new();
        loop
        {
            match self.info.content.chars().nth(self.index)
            {
                Some(x) => {
                    if x != ' ' {
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
                    return Ok(self.advance_with_token(Type::Semi, ';'.to_string()))
                }
                Some(',') => {
                    return Ok(self.advance_with_token(Type::Comma, ','.to_string()));
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

impl PFuncs for Parser
{
    fn new_parser(lexer: Lexer) -> Self
    {
        Self {
            lex: lexer
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

                        loop {
                            match self.lex.token
                            {
                                Type::NUM => {
                                    self.get_next_token();
                                },
                                Type::Comma => {
                                    self.get_next_token();
                                },
                                Type::T_RB => {
                                    break;
                                }
                                _ => return Err(PError::unexpected_token(self.lex.token.clone()))
                            }
                        }
                        self.get_next_token();

                        match self.lex.token
                        {
                            Type::Semi => {
                                return Ok(self.clone());
                            },
                            Type::EOF => {
                                return Ok(self.clone());
                            },
                            _ => return Err(PError::unexpected_token(self.lex.token.clone()))
                        }
                    },
                    _ => return Err(PError::unexpected_token(self.lex.token.clone())),
                }

            }
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
                        Ok(_) => {
                        },
                        Err(t) => {
                            return Err(t);
                        }
                    }
                    continue;
                },
                Type::K_PRINT => {
                    println!("Printing!");
                    break;
                },
                _ => break
            }
        }

        Ok(self.clone())
    }
}
