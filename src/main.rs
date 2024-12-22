fn main() {
    let s = "Hello world";
    println!("source: {}, parsed: {:?}", s, source(s));

    let s = "(123 456 ) world";
    println!("source: {}, parsed: {:?}", s, source(s));

    let s = "((car cdr) cdr)";
    println!("source: {}, parsed: {:?}", s, source(s));

    let s = "()())))((()))";
    println!("source: {}, parsed: {:?}", s, source(s));
}

fn advance_char(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

fn peek_char(input: &str) -> Option<char> {
    input.chars().next()
}

fn source(mut input: &str) -> (&str, TokenTree) {
    let mut tokens = vec![];
    while !input.is_empty() {
        input = if let Some((next_input, token)) = token(input) {
            match token {
                Token::LParen => {
                    let (next_input, tt) = source(next_input);
                    tokens.push(tt);
                    next_input
                }
                Token::RParen => return (next_input, TokenTree::Tree(tokens)),
                _ => {
                    tokens.push(TokenTree::Token(token));
                    next_input
                }
            }
        } else {
            break;
        }
    }
    (input, TokenTree::Tree(tokens))
}

#[derive(Debug, PartialEq)]
enum Token<'src> {
    Ident(&'src str),
    Number(f64),
    LParen,
    RParen,
}

#[derive(Debug, PartialEq)]
enum TokenTree<'src> {
    Token(Token<'src>),
    Tree(Vec<TokenTree<'src>>),
}

fn token(input: &str) -> Option<(&str, Token)> {
    let input = whitespace(input);
    if let Some(res) = ident(input) {
        Some(res)
    } else if let Some(res) = number(input) {
        Some(res)
    } else if let Some(res) = lparen(input) {
        Some(res)
    } else if let Some(res) = rparen(input) {
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

fn ident(mut input: &str) -> Option<(&str, Token)> {
    let start = input;
    if let Some(_x @ ('a'..='z' | 'A'..='Z')) = peek_char(input) {
        input = advance_char(input);
        while let Some(_x @ ('a'..='z' | 'A'..='Z' | '0'..='9')) = peek_char(input) {
            input = advance_char(input);
        }
        Some((input, Token::Ident(&start[..(start.len() - input.len())])))
    } else {
        None
    }
}

fn number(mut input: &str) -> Option<(&str, Token)> {
    let start = input;
    if let Some(_x @ ('-' | '+' | '.' | '0'..='9')) = peek_char(input) {
        input = advance_char(input);
        while let Some(_x @ ('.' | '0'..='9')) = peek_char(input) {
            input = advance_char(input);
        }
        if let Ok(num) = start[..(start.len() - input.len())].parse() {
            Some((input, Token::Number(num)))
        } else {
            None
        }
    } else {
        None
    }
}

fn lparen(mut input: &str) -> Option<(&str, Token)> {
    if let Some('(') = peek_char(input) {
        input = advance_char(input);
        Some((input, Token::LParen))
    } else {
        None
    }
}

fn rparen(mut input: &str) -> Option<(&str, Token)> {
    if let Some(')') = peek_char(input) {
        input = advance_char(input);
        Some((input, Token::RParen))
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
        assert_eq!(ident("Adam"), Some(("", Token::Ident("Adam"))));
    }

    #[test]
    fn test_number() {
        assert_eq!(number("123.45 "), Some((" ", Token::Number(123.45))));
    }
}
