mod src;

use src::starter;
use starter::FileInfo;
use starter::Funcs;

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
                                                        
                                                    },
                                                    Err(e) => panic!("{:?}", e)
                                                }
                                            }
                                        }
                                        //break;
                                    }
                                    Err(token) => panic!("{:?}", token)
                                }
                            }
                        },
                        Err(E) => panic!("{:?}", E)
                    }
                },
                Err(T) => panic!("{:?}", T)
            }
        },
        Err(t) => panic!("{:?}", t)
    }
}
