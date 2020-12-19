#[derive(Debug)]
enum Token {
    Num(usize),
    Sum,
    Mul,
    LParen,
    RParen,
}

fn tokenize<'a>(input: &'a str) -> impl Iterator<Item = Token> + 'a {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '0'..='9' => Token::Num(c.to_digit(10).unwrap() as usize),
            '+' => Token::Sum,
            '*' => Token::Mul,
            '(' => Token::LParen,
            ')' => Token::RParen,
            _ => unimplemented!("tokenize: unsupported char"),
        })
}

/// Evaluate expression left-to-right without any operator precedence.
/// Parenthesis are respected and change the evaluation order by first
/// evaluating the sub-expression in the parenthesis.
fn evaluate(tokens: &mut impl Iterator<Item = Token>) -> usize {
    let mut res = 0;

    while let Some(t) = tokens.next() {
        match t {
            Token::LParen => {
                // `LParen` either at beginning of expr (lhs) or on
                // rhs and then it is handled by the operators.
                assert_eq!(res, 0);
                res = evaluate(tokens);
            }
            Token::Num(n) => {
                // `Num` either at beginning of expr (lhs) or on rhs
                // and then it is handled by the operators.
                assert_eq!(res, 0);
                res = n;
            }
            Token::Mul => match tokens.next() {
                Some(Token::Num(n)) => res *= n,
                Some(Token::LParen) => res *= evaluate(tokens),
                _ => unimplemented!(),
            },
            Token::Sum => match tokens.next() {
                Some(Token::Num(n)) => res += n,
                Some(Token::LParen) => res += evaluate(tokens),
                _ => unimplemented!(),
            },
            Token::RParen => break,
        }
    }

    res
}

/// Evaluate expression left-to-right giving `+` operator prcendence
/// over `*` operator.
/// Parenthesis are respected and change the evaluation order by first
/// evaluating the sub-expression in the parenthesis.
fn evaluate2(tokens: &mut impl Iterator<Item = Token>) -> usize {
    let mut res = 0;

    while let Some(t) = tokens.next() {
        match t {
            Token::LParen => {
                // `LParen` either at beginning of expr (lhs) or on
                // rhs and then it is handled by the operators.
                assert_eq!(res, 0);
                res = evaluate2(tokens);
            }
            Token::Num(n) => {
                // `Num` either at beginning of expr (lhs) or on
                // rhs and then it is handled by the operators.
                assert_eq!(res, 0);
                res = n;
            }
            Token::Mul => {
                // To give `+` precedence over `*` we treat rhs as
                // sub-expression by evaluating rhs first.
                res *= evaluate2(tokens);
                break;
            }
            Token::Sum => match tokens.next() {
                Some(Token::Num(n)) => res += n,
                Some(Token::LParen) => res += evaluate2(tokens),
                _ => unimplemented!(),
            },
            Token::RParen => break,
        }
    }

    res
}

fn challenge1() -> usize {
    aoc20::read_input_to_string("day18")
        .lines()
        .map(|expr| evaluate(&mut tokenize(expr)))
        .sum()
}

fn challenge2() -> usize {
    aoc20::read_input_to_string("day18")
        .lines()
        .map(|expr| evaluate2(&mut tokenize(expr)))
        .sum()
}

fn main() {
    println!("{}", challenge1());
    println!("{}", challenge2());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_challenge1() {
        assert_eq!(challenge1(), 6923486965641);
    }

    #[test]
    fn check_challenge2() {
        assert_eq!(challenge2(), 70722650566361);
    }

    #[test]
    fn examples1() {
        let eval = |e| evaluate(&mut tokenize(e));
        assert_eq!(eval("2 * 3 + (4 * 5)"), 26);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12240);
        assert_eq!(
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn examples2() {
        let eval = |e| evaluate2(&mut tokenize(e));
        assert_eq!(eval("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(eval("2 * 3 + (4 * 5)"), 46);
        assert_eq!(eval("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(eval("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
        assert_eq!(
            eval("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
