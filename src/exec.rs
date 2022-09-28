// exec.rs

use crate::read;
pub struct Env;

pub fn eval_str(env: &mut Env, s: &str) -> Result<String, ()> {
    read::read(&format!("(\n{})", s));
    Ok(s.to_string())
}