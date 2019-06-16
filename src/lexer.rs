//! For making notable symbols and words out of text.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    LParen,
    RParen,
    Pipe,
    Equals,
}
use self::Operator::*;

/// All functions assume the next factor immediately following to be their argument.
/// Functions cannot contain more than a single argument. This may be changed in the future.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Function {
    // mul 2,3
    Sqrt,
    Sin,
    Cos,
    Tan,
    Log,
    Abs,
}
use self::Function::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Constant {
    Pi,
    E
}
use self::Constant::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    Function(Function),
    Constant(Constant),
    /// Identifiers are placeholders for values. These are meant to be replaced
    /// with the `.replace` methon on `Expr`s. Identifiers present in an ast
    /// will cause errors at computation time.
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    InvalidCharacter(char),
    InvalidNumber(String),
}

/// Turn a string into a vector of tokens. This function generally takes the most time,
/// compared to parsing and computing. It is best to run this function as few times as
/// reasonably possible.
/// ```
/// let tokens = tokenize("2 + 2").unwrap();
/// assert_eq!(tokens.as_slice(), &[Token::Number(2.0), Token::Operator(Operator::Plus), Token::Number(2.0)]);
/// ```
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::<Token>::new();

    let chars: Vec<char> = input.chars().collect();

    let mut i = 0usize;
    while i < chars.len() {
        match chars[i] {
            '+' => tokens.push(Token::Operator(Plus)),
            '-' => tokens.push(Token::Operator(Minus)),
            '*' => tokens.push(Token::Operator(Star)),
            '/' => tokens.push(Token::Operator(Slash)),
            '%' => tokens.push(Token::Operator(Percent)),
            '^' => tokens.push(Token::Operator(Caret)),
            '(' => tokens.push(Token::Operator(LParen)),
            ')' => tokens.push(Token::Operator(RParen)),
            '|' => tokens.push(Token::Operator(Pipe)),
            '=' => tokens.push(Token::Operator(Equals)),
            c => {
                if c.is_whitespace() {
                    i += 1;
                    continue;
                } else if c.is_digit(10) || c == '.' {
                    let mut number_string = c.to_string(); // Like creating a new string and pushing the character.
                    
                    i += 1;
                    while i < chars.len() && (chars[i].is_digit(10) || chars[i] == '.') {
                        number_string.push(chars[i]);
                        i += 1;
                    }

                    match number_string.parse::<f64>() {
                        Ok(num) => tokens.push(Token::Number(num)),
                        _ => return Err(LexerError::InvalidNumber(number_string)),
                    }

                    continue; // We i += 1 at end of latest while.
                } else if c.is_alphabetic() {
                    let mut full_identifier = c.to_string();

                    i += 1; // Step over first character of identifier.
                    // While we're still reading alphabetical characters.
                    while i < chars.len() && chars[i].is_alphabetic() {
                        full_identifier.push(chars[i]);
                        i += 1;
                    }

                    match &full_identifier.to_lowercase()[..] {
                        // Constants
                        "pi" => tokens.push(Token::Constant(Pi)),
                        "e" => tokens.push(Token::Constant(E)),

                        // Functions
                        "sqrt" => tokens.push(Token::Function(Sqrt)),
                        "sin" => tokens.push(Token::Function(Sin)),
                        "cos" => tokens.push(Token::Function(Cos)),
                        "tan" => tokens.push(Token::Function(Tan)),
                        "log" => tokens.push(Token::Function(Log)),
                        "abs" => tokens.push(Token::Function(Abs)),
                        
                        id => tokens.push(Token::Identifier(id.to_owned())),
                    }

		            continue;
                } else {
                    return Err(LexerError::InvalidCharacter(c));
                }
            }
        }
        i += 1;
    }
    
    tokens.shrink_to_fit();
    Ok(tokens)
}
