// read.rs

// Lexer ///////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone)]
pub enum Tok {
    NewLine(),
    Paren(char),
    Str(String),
    Key(String),
    Int(i32),
    Flt(f32),
}

fn lex(code: &str) -> Vec<Tok> {
    let mut tokens: Vec<Tok> = vec![];
    let mut iter = code.chars().peekable();

    while let Some(c) = iter.next() {
        match c {
            ',' => {
                // comment
                while let Some(next) = iter.peek() {
                    match next {
                        '\n' => break,
                        _ => iter.next()
                    };
                }
            }
            '\n' | ';' => {
                // new line
                tokens.push(Tok::NewLine())
            }
            '(' | ')' => {
                // parenthesis
                tokens.push(Tok::Paren(c))
            }
            '\'' => {
                // string
                let mut s = String::new();
                while let Some(ch) = iter.next() {
                    match ch {
                        '\\' if iter.peek() == Some(&'\'') =>
                            s.push(iter.next().expect("")),
                        '\'' => break,
                        _ => s.push(ch)
                    }
                }
                tokens.push(Tok::Str(s))
            }
            '!'..='~' => {
                // number or keyword
                let mut s = c.to_string();
                while let Some(ch) = iter.peek() {
                    match ch {
                        '!'..='&' | '*'..='+' | '-'..=':' | '<'..='~' => {
                            s.push(*ch);
                            iter.next();
                        },
                        _ => break
                    }
                }
                tokens.push(num_or_key(s))
            }
            _ => ()
        }
    }
    println!("{:?}", tokens);
    return tokens;
}

fn num_or_key(s: String) -> Tok {
    if s.starts_with(|c: char| c == '-' || c == '.' || c.is_ascii_digit())
        && s.find(|c: char| c.is_ascii_digit()).is_some()
    {
        if s.contains('.') {
            return Tok::Flt(s.parse::<f32>().expect("Failed to parse as float"))
        }
        return Tok::Int(s.parse::<i32>().expect("Failed to parse as integer"))
    }
    return Tok::Key(s);
}

// Parser //////////////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Atom(Tok),
    Expr(Vec<Expr>)
}

fn parse(tokens: Vec<Tok>) -> Expr {
    let mut iter = tokens.iter().peekable();
    return parse_expr(&mut iter);
}

fn parse_expr(iter: &mut core::iter::Peekable<core::slice::Iter<Tok>>) -> Expr {
    if let Some(t) = iter.next() {
        match t {
            Tok::Paren('(') => {
                let mut vec = vec![];
                
                if iter.peek() == Some(&&Tok::NewLine()) {
                    // multi-expression
                    iter.next(); // advance NL
                    let mut inner_vec = vec![];

                    while let Some(new_t) = iter.peek() {
                        match new_t {
                            Tok::NewLine() | Tok::Paren(')') => {
                                if !inner_vec.is_empty() {
                                    vec.push(Expr::Expr(inner_vec.clone()));
                                    inner_vec.clear();
                                }
                                if new_t == &&Tok::Paren(')') {
                                    break;
                                }
                                iter.next();
                            }
                            _ => inner_vec.push(parse_expr(iter))
                        }
                    }
                } else {
                    // single expression
                    while let Some(new_t) = iter.peek() {
                        match new_t {
                            Tok::NewLine() => {iter.next();}
                            Tok::Paren(')') => break,
                            _ => vec.push(parse_expr(iter))
                        }
                    }
                }
                iter.next(); // advance ')'
                return Expr::Expr(vec);
            },
            Tok::Paren(')') => panic!("Missing closing parenthesis"),
            _ => Expr::Atom(t.clone())
        }
    } else {
        // empty expr (== nil)
        return Expr::Expr(vec![]);
    }
}

// Reader //////////////////////////////////////////////////////////////////////

pub fn read(code: &str) -> Expr {
    let ast = parse(lex(code));
    println!("{:?}", ast);
    return ast;
}