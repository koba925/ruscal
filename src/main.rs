fn main() {
    let s = "123";
    println!("source: {}, parsed: {:?}", s, expr(s));

    let s = "Hello + world";
    println!("source: {}, parsed: {:?}", s, expr(s));

    let s = "(123 + 456 ) + world";
    println!("source: {}, parsed: {:?}", s, expr(s));

    let s = "car + cdr + cdr";
    println!("source: {}, parsed: {:?}", s, expr(s));

    let s = "((1 + 2) + (3 + 4)) + 5 + 6";
    println!("source: {}, parsed: {:?}", s, expr(s));
}

fn advance_char(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

fn peek_char(input: &str) -> Option<char> {
    input.chars().next()
}

#[derive(Debug, PartialEq)]
enum Expression<'src> {
    Ident(&'src str),
    NumLiteral(f64),
    Add(Box<Expression<'src>>, Box<Expression<'src>>),
}

fn expr(input: &str) -> Option<(&str, Expression)> {
    if let Some(res) = add(input) {
        Some(res)
    } else if let Some(res) = term(input) {
        Some(res)
    } else {
        None
    }
}

fn paren(input: &str) -> Option<(&str, Expression)> {
    let next_input = lparen(whitespace(input))?;
    let (next_input, expr) = expr(next_input)?;
    let next_input = rparen(whitespace(next_input))?;
    Some((next_input, expr))
}

fn add_term(input: &str) -> Option<(&str, Expression)> {
    let (next_input, lhs) = term(input)?;
    let next_input = plus(whitespace(next_input))?;
    Some((next_input, lhs))
}

fn add(mut input: &str) -> Option<(&str, Expression)> {
    let mut left = None;
    while let Some((next_input, expr)) = add_term(input) {
        if let Some(prev_left) = left {
            left = Some(Expression::Add(Box::new(prev_left), Box::new(expr)));
        } else {
            left = Some(expr);
        }
        input = next_input;
    }
    let left = left?;
    let (next_input, rhs) = expr(input)?;
    Some((next_input, Expression::Add(Box::new(left), Box::new(rhs))))
}

fn term(input: &str) -> Option<(&str, Expression)> {
    if let Some(res) = paren(input) {
        Some(res)
    } else if let Some(res) = token(input) {
        Some(res)
    } else {
        None
    }
}

fn token(input: &str) -> Option<(&str, Expression)> {
    let input = whitespace(input);
    if let Some(res) = ident(input) {
        Some(res)
    } else if let Some(res) = number(input) {
        Some(res)
    } else {
        None
    }
}

fn whitespace(mut input: &str) -> &str {
    while let Some(' ') = peek_char(input) {
        input = advance_char(input);
    }
    input
}

fn ident(mut input: &str) -> Option<(&str, Expression)> {
    let start = input;
    if let Some(_x @ ('a'..='z' | 'A'..='Z')) = peek_char(input) {
        input = advance_char(input);
        while let Some(_x @ ('a'..='z' | 'A'..='Z' | '0'..='9')) = peek_char(input) {
            input = advance_char(input);
        }
        Some((
            input,
            Expression::Ident(&start[..(start.len() - input.len())]),
        ))
    } else {
        None
    }
}

fn number(mut input: &str) -> Option<(&str, Expression)> {
    let start = input;
    if let Some(_x @ ('-' | '+' | '.' | '0'..='9')) = peek_char(input) {
        input = advance_char(input);
        while let Some(_x @ ('.' | '0'..='9')) = peek_char(input) {
            input = advance_char(input);
        }
        if let Ok(num) = start[..(start.len() - input.len())].parse() {
            Some((input, Expression::NumLiteral(num)))
        } else {
            None
        }
    } else {
        None
    }
}

fn lparen(input: &str) -> Option<&str> {
    if let Some('(') = peek_char(input) {
        Some(advance_char(input))
    } else {
        None
    }
}

fn rparen(input: &str) -> Option<&str> {
    if let Some(')') = peek_char(input) {
        Some(advance_char(input))
    } else {
        None
    }
}

fn plus(input: &str) -> Option<&str> {
    if let Some('+') = peek_char(input) {
        Some(advance_char(input))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_whitespace() {
        assert_eq!(whitespace("   "), "");
    }

    #[test]
    fn test_ident() {
        assert_eq!(ident("Adam"), Some(("", Expression::Ident("Adam"))));
    }

    #[test]
    fn test_number() {
        assert_eq!(
            number("123.45 "),
            Some((" ", Expression::NumLiteral(123.45)))
        );
    }
}
