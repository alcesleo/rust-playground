use infix_parser::{parse, Token, OpInfo};

// Numbers and right-parens should never be on the stack
enum StackToken {
    Operator(OpInfo),
    LeftParen,
}

pub fn infix_to_postfix(input: &str) -> String {
    let mut result = Vec::new();
    let mut stack  = Vec::new();

    for token in parse(input) {
        match token {
            Token::Number(n)    => result.push(n.to_string()),
            Token::LeftParen    => stack.push(StackToken::LeftParen),
            Token::RightParen   => pop_until_paren(&mut stack, &mut result),
            Token::Operator(op) => push_under_prioritized(&mut stack, &mut result, op),
        }
    }

    // Pop the rest of the stack onto the result
    while let Some(token) = stack.pop() {
        match token {
            StackToken::Operator(op) => result.push(op.sign),
            StackToken::LeftParen    => panic!("unbalanced parentheses"),
        }
    }

    result.join(" ")
}

// Pops the stack onto the result until it runs out or a ( is found
fn pop_until_paren(stack: &mut Vec<StackToken>, result: &mut Vec<String>) {
    while let Some(StackToken::Operator(op)) = stack.pop() {
        result.push(op.sign)
    }
}

// Pops higher priority tokens onto the result before pushing op onto the stack
fn push_under_prioritized(mut stack: &mut Vec<StackToken>, result: &mut Vec<String>, op: OpInfo) {
    while should_pop_before_pushing_operator(&mut stack, &op) {
        match stack.pop().unwrap() {
            StackToken::Operator(op) => result.push(op.sign),
            _ => panic!("parentheses found where there should never be one")
        }
    }

    stack.push(StackToken::Operator(op));
}

// Returns whether operators needs to be popped off the stack before
// token is put on the stack with regards to operator precedence and
// associativity
fn should_pop_before_pushing_operator(stack: &mut Vec<StackToken>, token: &OpInfo) -> bool {
    return match stack.last() {
        Some(&StackToken::Operator(ref top)) => {
            !(top.precedence < token.precedence
              || (top.precedence == token.precedence && token.right_associative))
        }
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_infix_to_postfix() {
        assert_eq!("1 2 3 * +", infix_to_postfix("1 + 2 * 3"));
    }

    #[test]
    fn it_converts_infix_to_postfix_with_parentheses() {
        assert_eq!("5 1 2 + 4 * + 3 -", infix_to_postfix("5 + ( 1 + 2 ) * 4 - 3"));
    }

    #[test]
    #[should_panic (expected = "unbalanced parentheses")]
    fn it_detects_unbalanced_parentheses() {
        infix_to_postfix("5 * ( 6 + 3");
    }
}
