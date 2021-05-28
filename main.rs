mod src;

use src::starter;
use starter::FileInfo;
use starter::Funcs;

use src::lexer;
use lexer::Lexer;
use lexer::LFuncs;

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
                            println!("{:?}, {:?}", I.clone(), lexer);
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
