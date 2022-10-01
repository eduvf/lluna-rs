// main.rs

mod read;
mod repl;
mod exec;
use exec::Env;

fn main() {
    let mut env = exec::Env::new();
    // repl::repl(&mut env);

    let test = "
    ~ f n (	, factorial
        : r
        ? (< 0 n) (
            : r (* n (f (- n 1)))
        )(
            : r 1
        )
    )
    ! (f 5)
    ";
    exec::eval_str(&mut env, test);
}
