fn main() {
    let input = "123 world";
    println!("source: {}, parsed: {:?}", input, source(input));

    let input = "Hello world";
    println!("source: {}, parsed: {:?}", input, source(input));

    let input = "   world";
    println!("source: {}, parsed: {:?}", input, source(input));
}

fn source(mut input: &str) -> Vec<Token> {
    let mut tokens = vec![];
    while !input.is_empty() {
        input = if let (next_input, Some(token)) = token(input) {
            tokens.push(token);
            next_input
        } else {
            break;
        }
    }
    tokens
}
#[derive(Debug, PartialEq, Eq)]
enum Token {
    Ident = 0,
    Number = 1,
}

fn token(i: &str) -> (&str, Option<Token>) {
    if let (i, Some(ident_res)) = ident(whitespace(i)) {
        (i, Some(ident_res))
    } else if let (i, Some(number_res)) = number(whitespace(i)) {
        (i, Some(number_res))
    } else {
        (i, None)
    }
}

fn whitespace(input: &str) -> &str {
    let mut cs = input.chars();
    while let Some(' ') = cs.clone().next() {
        cs.next();
    }
    cs.as_str()
}

fn number(input: &str) -> (&str, Option<Token>) {
    let mut cs = input.chars();
    if let Some(_x @ ('-' | '+' | '.' | '0'..='9')) = cs.clone().next() {
        cs.next();
        println!("{}", _x);
        while let Some(_x @ ('.' | '0'..='9')) = cs.clone().next() {
            cs.next();
            println!("{}", _x);
        }
        (cs.as_str(), Some(Token::Number))
    } else {
        (cs.as_str(), None)
    }
}

fn ident(input: &str) -> (&str, Option<Token>) {
    let mut cs = input.chars();
    if let Some(_x @ ('a'..='z' | 'A'..='Z')) = cs.clone().next() {
        cs.next();
        println!("{}", _x);
        while let Some(_x @ ('a'..='z' | 'A'..='Z' | '0'..='9')) = cs.clone().next() {
            cs.next();
            println!("{}", _x);
        }
        (cs.as_str(), Some(Token::Ident))
    } else {
        (cs.as_str(), None)
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
        assert_eq!(ident("Adam"), ("", Some(Token::Ident)));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("123.45 "), (" ", Some(Token::Number)));
    }
}
