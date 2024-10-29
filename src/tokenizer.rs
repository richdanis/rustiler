use std::fmt;
pub struct Token {
    pub value: String,
    pub line: i32
}

impl Token {
    pub fn build(value: &str, line: i32) -> Token {
        Token {
            value: String::from(value),
            line: line
        }
    }
}

// printing token for debugging purposes
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({}, line: {}, length: {})", self.value, self.line, self.value.len())
    }
}

// we let push take ownership of value, as we don't need
// value after push anymore (we just clean it)
fn push(tokens: &mut Vec<Token>, value: String, line: i32) -> Result<String, &'static str> {

    if !value.is_empty() {

        let token = Token {
            value: value,
            line: line
        };

        tokens.push(token);
        Ok(String::new())
    }
    else {
        Ok(value)
    }

}

pub fn tokenize(code: String) -> Result<Vec<Token>, &'static str> {

    // tokenize the code
    // iterate over code
    let mut token_string = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 0;

    for c in code.chars() {

        match c {
            '\n' => {
                token_string = push(&mut tokens, token_string, line)?;
                line += 1;
            },
            '\r' | ' ' => {
                token_string = push(&mut tokens, token_string, line)?;
            },
            '{' | '}' | '(' | ')' | ';' => {
                token_string = push(&mut tokens, token_string, line)?;
                push(&mut tokens, c.to_string(), line)?;
            },
            _ => {
                token_string.push(c);
            }
        }

    }

    if !token_string.is_empty() {
        push(&mut tokens, token_string, line)?;
    }
        
    Ok(tokens)

}