mod infix_to_postfix;

pub fn postfix(input: &str) -> i32 {
    let mut stack = Vec::new();
    
    for token in input.split_whitespace() {
        match parse(token) {
            Token::Operand(n)   => stack.push(n),
            Token::Operator(op) => perform_operation(&mut stack, &*op),
        }
    }

    if stack.len() > 1 {
        panic!("not enough operators");
    }

    stack.pop().unwrap()
}

fn perform_operation(stack: &mut Vec<i32>, operator: &str) {
    let rhs = stack.pop().expect("not enough operands");
    let lhs = stack.pop().expect("not enough operands");

    let result = match operator {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => lhs / rhs,
        "^" => lhs.pow(rhs as u32),
        _   => panic!("unsupported operator")
    };

    stack.push(result);
}

fn parse(token: &str) -> Token {
    match token.parse() {
        Ok(n)  => Token::Operand(n),
        Err(_) => Token::Operator(token.to_string()),
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Operator(String),
    Operand(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_evaluates_postfix() {
        assert_eq!(14, postfix("5 1 2 + 4 * + 3 -"));
    }

    #[test]
    fn it_correctly_evaluates_postfix_2() {
        assert_eq!(16, postfix("2 5 ^ 2 /"));
    }

    #[test]
    #[should_panic(expected = "unsupported operator")]
    fn it_reports_error_when_using_unsupported_operator() {
        postfix("1 2 =");
    }

    #[test]
    #[should_panic(expected = "not enough operators")]
    fn it_reports_error_when_not_having_a_single_result_at_the_end() {
        postfix("1 2 3 +");
    }

    #[test]
    #[should_panic(expected = "not enough operands")]
    fn it_reports_error_when_having_too_many_operators_and_even_stack() {
        postfix("+");
    }

    #[test]
    #[should_panic(expected = "not enough operands")]
    fn it_reports_error_when_having_too_many_operators_and_odd_stack() {
        postfix("1 +");
    }
}
