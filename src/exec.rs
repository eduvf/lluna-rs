// exec.rs

use std::collections::HashMap;

use crate::read::{self, Expr};
pub struct Env<'a> {
    map: HashMap<String, Expr>,
    out: Option<&'a Env<'a>>
}
impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        let mut env_map = HashMap::new();
        // TODO: add std functions
        Env::<'a>{map: env_map, out: None}
    } 
    pub fn list(&self) {
        for (k, v) in &self.map {
            println!("{}:{:?}", k, v)
        }
    }
    pub fn set(&mut self, key: String, val: Expr) {
        let entry = self.map.entry(key).or_insert_with(|| val.clone());
        if val != *entry {*entry = val}
    }
    pub fn get(&self, key: &str) -> Expr {
        match self.map.get(key) {
            Some(val) => val.clone(),
            None => {
                match self.out {
                    Some(out_env) => out_env.get(key),
                    None => Expr::Expr(vec![]) // return nil
                }
            }
        }
    }
}

pub fn eval_str(env: &mut Env, s: &str) -> Result<String, ()> {
    read::read(&format!("(\n{})", s));
    Ok(s.to_string())
}