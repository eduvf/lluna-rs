// read.rs

#[derive(Debug)]
pub enum Tok {
    NewLine(),
    Paren(char),
    Str(String),
    Key(String),
    Int(i32),
    Flt(f32),
}

pub fn lex(code: &str) -> Vec<Tok> {
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
                        '\\' => s.push(iter.next().expect("Escaped EOF")),
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
    if s.chars().all(|c| c == '-' || c == '.' || c.is_ascii_digit()) {
        if s.contains('.') {
            return Tok::Flt(s.parse::<f32>().expect("Failed to parse as float"))
        }
        return Tok::Int(s.parse::<i32>().expect("Failed to parse as integer"))
    }
    return Tok::Key(s);
}