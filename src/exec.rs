// exec.rs

use std::collections::HashMap;
use crate::read;
// pub struct Env<'a> {
//     data: HashMap<String, Expr>,
//     outer: Option<&'a Env<'a>>
// }
pub struct Env;

pub fn eval_str(env: &mut Env, s: &str) -> Result<String, ()> {
    read::lex(&s);
    Ok(s.to_string())
} 