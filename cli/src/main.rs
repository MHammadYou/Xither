use std::{ fs, env };

use parser::Lexer;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let filename = &args[1];
        let source = fs::read_to_string(filename);

        if let Ok(source) = source {
            let tokens = Lexer::new(&source).get_tokens();
            
            match tokens {
                Ok(tokens) => println!("{:#?}", tokens),
                Err(error) => println!("{}", error),
            }
        }
    } else {
        eprintln!("Error: No filename provided")
    }

}