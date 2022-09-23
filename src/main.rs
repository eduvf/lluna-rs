// main.rs

mod read;
mod repl;
mod exec;
use exec::Env;

fn main() {
    let mut env = Env;
    repl::repl(&mut env);
}
