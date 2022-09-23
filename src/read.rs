// read.rs

use crate::exec::Env;
pub enum Expr {
    Int(i32),
    Flt(f32),
    Key(String),
    Char(char),
    List(Vec<Expr>),
    Func(fn(&[Expr]) -> Result<Expr, ()>)
}

pub fn eval_str(env: &mut Env, s: &str) -> Result<String, ()> {
    Ok(s.to_string())
} 