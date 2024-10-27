use std::fmt;
pub struct Token {
    value: String,
    line: i32,
    token_type: Token_Type
}

// printing token for debugging purposes
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({}, line: {}, length: {})", self.value, self.line, self.value.len())
    }
}

enum Token_Type {
    Keyword,
    Identifier,
    Number,
    Punctuation
}

// we let push take ownership of value, as we don't need
// value after push anymore (we just clean it)
fn push(tokens: &mut Vec<Token>, value: String, line: i32) -> String {

    if !value.is_empty() {

        let token = Token {
            value: value,
            line: line,
            token_type: Token_Type::Keyword
        };

        tokens.push(token);
        String::new()
    }
    else {
        value
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
                token_string = push(&mut tokens, token_string, line);
                line += 1;
            },
            '\r' | ' ' => {
                token_string = push(&mut tokens, token_string, line);
            },
            '{' | '}' | '(' | ')' | ';' => {
                token_string = push(&mut tokens, token_string, line);
                push(&mut tokens, c.to_string(), line);
            },
            _ => {
                token_string.push(c);
            }
        }

    }

    if !token_string.is_empty() {
        push(&mut tokens, token_string, line);
    }
        
    Ok(tokens)

}