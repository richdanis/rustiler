/* Simple formal grammar in Backus-Naur Form:

    <program> ::= <function>
    <function> ::= "int" <id> "(" ")" "{" <statement> "}"
    <statement> ::= "return" <exp> ";"
    <exp> ::= <int>

*/

use crate::tokenizer::Token;

pub struct Program {
    pub function: Function
}

pub struct Function {
    pub identifier: String,
    pub statement: Statement
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Statement {
    pub expression: Expression
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Expression {
    pub value: String
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Keyword {
    Return,
    Int
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    Keyword(Keyword),
    Identifier,
    Number,
    Punctuation
}

fn get_token_type(token: &Token) -> Result<TokenType, &'static str> {

    if token.value == "return" {
        return Ok(TokenType::Keyword(Keyword::Return));
    }

    if token.value == "int" {
        return Ok(TokenType::Keyword(Keyword::Int));
    }

    if token.value == "{" || token.value == "}" || token.value == "(" || token.value == ")" || token.value == ";" {
        return Ok(TokenType::Punctuation);
    }

    if is_number(&token.value) {
        return Ok(TokenType::Number);
    }

    if is_identifier(&token.value) {
        return Ok(TokenType::Identifier);
    }

    // this would only work if the returned error type is changed
    // return Err(format!("{} could not be recognized, syntax is violated.", token.value));
    return Err("Token could not be recognized, syntax is violated.")

}

fn is_number(token: &str) -> bool {
    
    if token.is_empty() {
        return false
    }
    let mut iter = token.chars();
    let first_char = iter.next().unwrap();
    if token.len() == 1 {
        return first_char.is_numeric()
    }

    return first_char != '0' && first_char.is_numeric() && iter.all(char::is_numeric);

}

fn is_identifier(token: &str) -> bool {

    return token.chars().all(char::is_alphabetic);
    
}

pub fn parse_program(tokens: &mut Vec<Token>) -> Result<Program, &'static str> {
    
    let function = parse_function(tokens)?;
    Ok(
        Program{
            function: function
        }
    )
}

pub fn parse_function(tokens: &mut Vec<Token>) -> Result<Function, &'static str> {
    
    let mut token = tokens.remove(0);
    let mut token_type = get_token_type(&token)?;

    if TokenType::Keyword(Keyword::Int) != token_type {
        panic!("First token of function should be \"int\"");
    }

    token = tokens.remove(0);
    token_type = get_token_type(&token)?;

    if TokenType::Identifier != token_type {
        panic!("Second token of function should be \"identifier\"");
    }

    let identifier = token.value.clone();

    token = tokens.remove(0);
    assert!(token.value == "(");
    token = tokens.remove(0);
    assert!(token.value == ")");

    token = tokens.remove(0);
    assert!(token.value == "{");

    let statement = parse_statement(tokens)?;

    return Ok(
        Function{
            identifier: identifier,
            statement: statement
        }
    )


}

// for now a statement consists of "return", "number" and ";"
pub fn parse_statement(tokens: &mut Vec<Token>) -> Result<Statement, &'static str> {

    let mut token = tokens.remove(0);
    let mut token_type = get_token_type(&token)?;

    if TokenType::Keyword(Keyword::Return) != token_type {
        panic!("First token of statement should be \"return\"");
    }

    let expression = parse_expression(tokens)?;

    token = tokens.remove(0);
    token_type = get_token_type(&token)?;

    if TokenType::Punctuation != token_type || token.value != ";" {
        panic!("Final token of statement should be \"}}\"");
    }

    Ok (
        Statement {
            expression: expression
        }
    )

}

pub fn parse_expression(tokens: &mut Vec<Token>) -> Result<Expression, &'static str> {
    
    let token = tokens.remove(0);
    let token_type = get_token_type(&token)?;

    if token_type != TokenType::Number {
        return Err("Expression can only contain numbers for now!")
    }

    Ok(
        Expression { 
            value: token.value 
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenizer::tokenize;

    #[test]
    fn numbers() {
        assert!(!is_number(""));
        assert!(!is_number("0128734"));
        assert!(!is_number("2724er"));
        assert!(!is_number("239 923"));
        assert!(is_number("0"));
        assert!(is_number("1829"));
    }

    #[test]
    fn identifiers() {
        assert!(!is_identifier(""));
        assert!(!is_identifier("89fjoe"));
        assert!(!is_identifier("yo mama"));
        assert!(is_identifier("yo_mama_3"));
    }

    #[test]
    fn token_types() {

        let mut token = Token::build("tugvuiz", 0);
        assert_eq!(TokenType::Identifier, get_token_type(&token).unwrap());

        token = Token::build("3414", 0);
        assert_eq!(TokenType::Number, get_token_type(&token).unwrap());

    }

    #[test]
    fn expressions() {

        let mut tokens: Vec<Token> = Vec::new();
        tokens.push(Token::build("oiur", 0));

        let expression_1 = parse_expression(&mut tokens);
        assert_eq!(expression_1, Err("Expression can only contain numbers for now!"));

        tokens = Vec::new();
        let number = "378547";
        tokens.push(Token::build(number, 0));

        let expression_2 = parse_expression(&mut tokens).unwrap();
        assert_eq!(expression_2, Expression {value: String::from(number)});
        
    }

    #[test]
    fn statements() {
        
        let mut tokens: Vec<Token> = Vec::new();
        tokens.push(Token::build("return", 0));
        tokens.push(Token::build("34", 0));
        tokens.push(Token::build(";", 0));

        let statement = parse_statement(&mut tokens).unwrap();

        let expression = Expression {value: String::from("34")};
        assert_eq!(statement, Statement {expression: expression});

    }

    #[test]
    fn functions() {

    }

    #[test]
    fn programs() {

    }

    #[test]
    fn simple_program() {
        let program = "int main() {
                             return 2;
                             }";

        let mut tokens = tokenize(String::from(program)).unwrap();

        let ast = parse_program(&mut tokens).unwrap();

        assert_eq!(ast.function.identifier, "main");
        assert_eq!(ast.function.statement.expression.value, "2");

    }

    #[test]
    fn wrong_keyword() {
        let program = "int main() {
            retun 2;
            }";

        let mut tokens = tokenize(String::from(program)).unwrap();

        let ast = parse_program(&mut tokens).unwrap();
        
    }
}