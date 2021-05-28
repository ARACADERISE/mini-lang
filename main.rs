mod src;

use src::starter;
use starter::FileInfo;
use starter::Funcs;

use src::ast;
use ast::AstFuncs;

use src::lexer;
use lexer::Lexer;
use lexer::LFuncs;
use lexer::Type;

use src::parser;
use parser::Parser;
use parser::PFuncs;

#[allow(non_snake_case)]
fn main()
{
    match FileInfo::new()
    {
        Ok(mut t) => {
            match t.append("dude.t".to_string())
            {
                Ok(mut T) => {
                    match T.read_file() {
                        Ok(I) => {
                            let mut lexer = Lexer::new_lexer(I.clone());
                            let mut parser = Parser::new_parser(lexer.clone());

                            loop {
                                match lexer.lex() {
                                    Ok(token) => {
                                        match token {
                                            Type::EOF => break,
                                            _ => {
                                                match parser.parse(lexer.clone())
                                                {
                                                    Ok(mut p) => {
                                                        lexer = p.lex.clone();
                                                    },
                                                    Err(e) => println!("{:?}", e)
                                                }
                                            }
                                        }
                                        //break;
                                    },
                                    Err(token) => println!("{:?}", token)
                                }
                            }

                            parser.AST.go_through();
                        },
                        Err(E) => println!("{:?}", E)
                    }
                },
                Err(T) => println!("{:?}", T)
            }
        },
        Err(t) => println!("{:?}", t)
    }
}
