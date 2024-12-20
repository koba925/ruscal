fn main() {
    let source = "123 world";
    println!(
        "source: {}, parsed: {:?}",
        source,
        ident(whitespace(number(source)))
    );
}

fn whitespace(input: &str) -> &str {
    let mut cs = input.chars();
    while let Some(' ') = cs.next() {
        cs.next();
    }
    cs.as_str()
}

fn number(input: &str) -> &str {
    let mut cs = input.chars();
    if let Some(_x @ ('-' | '+' | '.' | '0'..='9')) = cs.next() {
        while let Some(_x @ ('.' | '0'..='9')) = cs.next() {
            cs.next();
        }
    }
    cs.as_str()
}

fn ident(input: &str) -> &str {
    let mut cs = input.chars();
    if let Some(_x @ ('a'..='z' | 'A'..='Z')) = cs.next() {
        while let Some(_x @ ('a'..='z' | 'A'..='Z' | '0'..='9')) = cs.next() {
            cs.next();
        }
    }
    cs.as_str()
}
