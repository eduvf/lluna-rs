// repl.rs

use rustyline::Editor;
use crate::read::eval_str;
use crate::exec::Env;

pub fn repl(env: &mut Env) {
    let mut readline: Editor<()> = Editor::<()>::new().expect("Failed to load editor");
    
    let history_path = "./lluna_history.txt";
    if readline.load_history(history_path).is_err() {
        println!("No history found.");
    }
    
    loop {
        let input = readline.readline("(lluna) ");
        
        match input {
            Ok(line) => {
                readline.add_history_entry(&line);
                match eval_str(env, &line) {
                    Ok(r) => println!("{}", r),
                    Err(e) => eprintln!("Error: {:?}", e)
                }
            }
            Err(_) => {
                println!("Bye!");
                break;
            }
        }
    }

    readline.save_history(history_path).unwrap_or_default();
}