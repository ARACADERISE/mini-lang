mod src;

use src::starter;
use starter::FileInfo;
use starter::Funcs;

use src::lexer;
use lexer::Lexer;
use lexer::LFuncs;
use lexer::Type;

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

                            loop {
                                match lexer.lex() {
                                    Ok(token) => {
                                        match token {
                                            Type::EOF => break,
                                            _ => {
                                                println!("{:?}", token);
                                            } // should never get here
                                        }
                                        //break;
                                    }
                                    Err(token) => panic!("{:?}", token)
                                }
                            }

                            println!("{:?}", lexer);
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
