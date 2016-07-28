pub enum Token {
    Operator(OpInfo),
    Number(i32),
    LeftParen,
    RightParen,
}

pub struct OpInfo {
    pub sign: String,
    pub precedence: i32,
    pub right_associative: bool,
}

pub fn parse(input: &str) -> Vec<Token> {
    input.split_whitespace().map(|token| parse_token(token)).collect()
}

fn parse_token(token: &str) -> Token {
    match token.parse() {
        Ok(n)  => Token::Number(n),
        Err(_) => {
            match token {
                "(" => Token::LeftParen,
                ")" => Token::RightParen,
                _   => Token::Operator(OpInfo {
                    sign: token.to_string(),
                    precedence: precedence(token),
                    right_associative: is_right_associative(token),
                }),
            }
        }
    }
}

fn precedence(token: &str) -> i32 {
    match token {
        "+" | "-" => 10,
        "*" | "/" => 20,
        "^"       => 30,
        _         => panic!("unsupported operator"),
    }
}

fn is_right_associative(token: &str) -> bool {
    token == "^" || token == "/" || token == "*"
}
