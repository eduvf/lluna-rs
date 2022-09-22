// main.rs

use rustyline::{Editor, error::ReadlineError};

struct Env;

fn repl(env: &mut Env) -> Result<(), ReadlineError> {

    let mut readline: Editor<()> = Editor::<()>::new()?;
    loop {
        let input = readline.readline("(lluna) ");
        
        match input {
            Ok(line) => {
                println!("{}", &line);
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(_) => {
                break;
            }
        }
    }
    Ok(())
}

fn main() {
    println!("Hello, world!");
    let mut env = Env;
    repl(&mut env);
}
