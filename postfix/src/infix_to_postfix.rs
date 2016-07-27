pub fn infix_to_postfix(input: &str) -> String {
    let mut result = Vec::new();
    let mut stack  = Vec::new();
    
    for token in input.split_whitespace() {
        match parse(token) {
            Token::Number(n)  => result.push(n.to_string()),
            Token::LeftParen  => stack.push(token.to_string()),
            Token::RightParen => {
                while let Some(top) = stack.pop() {
                    if top == "(" { break }
                    result.push(top);
                }
            },
            Token::Operator(op) => {
                while let Some(top) = stack.pop() {
                    
                    if precedence(&top) < precedence(&op)
                        || (precedence(&top) == precedence(&op)
                            && is_right_associative(&op))
                    {
                        stack.push(top);
                        break
                    }
                    result.push(top);
                }
                stack.push(op);

            }
        }
    }

    // Pop the rest of the stack onto the result
    stack.reverse();
    result.append(&mut stack);

    result.join(" ")
}

fn precedence(token: &str) -> i32 {
    match token {
        "+" | "-" => 1,
        "*" | "/" => 2,
        "^"       => 3,
        _         => 0 // ()
    }
}

fn is_right_associative(token: &str) -> bool {
    token == "^" || token == "/" || token == "*"
}

fn parse(token: &str) -> Token {
    match token.parse() {
        Ok(n)  => Token::Number(n),
        Err(_) => {
            match token {
                "(" => Token::LeftParen,
                ")" => Token::RightParen,
                _   => Token::Operator(token.to_string()),   
            }
            
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Operator(String),
    Number(i32),
    LeftParen,
    RightParen,
}
//     // While s is not empty:
//         // Let t = next token.
//             // If t is an operand, r = r + t;
//             // If t is an open parenthesis, push it.
//             // If t is a close parenthesis:
//                 // while top <> open parenthesis
//                     // r = r + top
//                     // pop
//                 // pop // removes the open parenthesis
//             // If t is an operator
//                 // while not (top is of lower precedence than t
//                 // OR t is right associative and top is of equal precedence)
//                     // r = r + top
//                     // pop
//                 // push t


// // 2.	If the stack is empty or contains a left parenthesis on top, push the incoming operator onto the stack.
// // 3.	If the incoming symbol is a left parenthesis, push it on the stack.
// // 4.	If the incoming symbol is a right parenthesis, pop the stack and print the operators until you see a left parenthesis. Discard the pair of parentheses.
// // 5.	If the incoming symbol has higher precedence than the top of the stack, push it on the stack.
// // 6.	If the incoming symbol has equal precedence with the top of the stack, use association. If the association is left to right, pop and print the top of the stack and then push the incoming operator. If the association is right to left, push the incoming operator.
// // 7.	If the incoming symbol has lower precedence than the symbol on the top of the stack, pop the stack and print the top operator. Then test the incoming operator against the new top of stack.
// // 8.	At the end of the expression, pop and print all operators on the stack. (No parentheses should remain.)


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_infix_to_postfix() {
        assert_eq!("10 5 - 3 +", infix_to_postfix("10 - 5 + 3"));
    }

    #[test]
    fn it_converts_infix_to_postfix_with_parentheses() {
        assert_eq!("1 2 3 4 * + * 5 +", infix_to_postfix("1 * ( 2 + 3 * 4 ) + 5"))
    }
}
