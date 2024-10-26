pub struct Token {
    value: String,
    line: i32,
    token_type: Token_Type
}

enum Token_Type {
    Keyword,
    Identifier,
    Number,
    Punctuation
}

fn push(tokens: &mut Vec<Token>, value: &mut String, line: i32) {

    if !value.is_empty() {

        let token = Token {
            value: value.clone(),
            line: line,
            token_type: Token_Type::Keyword
        };

        tokens.push(token);
        value.clear();

    }

}

pub fn tokenize(code: &String) -> Result<Vec<Token>, &'static str> {

    // tokenize the code
    // iterate over code
    let mut token_string = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut line = 0;

    for c in code.chars() {

        match c {
            '\n' => {
                push(&mut tokens, &mut token_string, line);
                line += 1;
            },
            '\r' | ' ' => {
                push(&mut tokens, &mut token_string, line);
            },
            '{' | '}' | '(' | ')' | ';' => {
                push(&mut tokens, &mut token_string, line);
                push(&mut tokens, &mut c.to_string(), line);
            },
            _ => {
                token_string.push(c);
            }
        }

    }

    if !token_string.is_empty() {
        tokens.push(token_string);
    }
        
    Ok(tokens)

}