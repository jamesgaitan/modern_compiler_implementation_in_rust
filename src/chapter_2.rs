/// A custom lexer written for learning purposes
use std::str::FromStr;

use regex::Regex;

/// A Token created by the lexer
#[derive(Debug, PartialEq)]
enum Token {
    Id(String),
    Num(i32),
    Real(f32),
    Boolean(bool),

    // Special Characters
    StaticString(String),
    Comma,
    Colon,
    Semicolon,
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    Lbrace,
    Rbrace,
    Period,
    Plus,
    Minus,
    Star,
    ForwardSlash,
    Equal,
    LessThan,
    GreaterThan,
    Ampersand,
    Bar,

    // Reserved Keywords
    If,
    Else,
    ElseIf,
    For,
    While,
    Function,
    Let,
    Int,
    Bool,
    Float,
    String,
    Char,
    Mut,

    Passthrough,
}

/// Reserved words not allowable to be used as identifiers
static RESERVED_WORDS: &'static [&str] = &[
    "if", "else", "elseif", "for", "while", "fn", "let", "int", "bool", "float", "string", "char",
    "mut", "True", "False",
];

/// Tokenize a string which is of the language of this project
fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut input_string = input.to_owned();
    let match_functions: Vec<fn(&str) -> (&str, Option<Token>)> = vec![
        match_id,
        match_num,
        match_if,
        match_real,
        match_boolean,
        match_static_string,
        match_comma,
        match_colon,
        match_semicolon,
        match_lparen,
        match_rparen,
        match_lbracket,
        match_rbracket,
        match_lbrace,
        match_rbrace,
        match_period,
        match_plus,
        match_minus,
        match_star,
        match_forwardslash,
        match_equal,
        match_lessthan,
        match_greaterthan,
        match_ampersand,
        match_bar,
        match_else,
        match_elseif,
        match_for,
        match_while,
        match_function,
        match_let,
        match_int,
        match_bool,
        match_float,
        match_string,
        match_char,
        match_mut,
        match_whitespace,
        match_error,
    ];

    while input_string != "" {
        for func in &match_functions {
            if let (output, Some(tok)) = func(&input_string) {
                input_string = output.clone().to_string();

                if tok != Token::Passthrough {
                    tokens.push(tok);
                }

                break;
            }
        }
    }

    return tokens;
}

// Match a general regex. Return the rest of the input after matching and the matched string
fn match_re(re: Regex, input: &str) -> (&str, &str) {
    if let Some(mat) = re.find(input) {
        return (&input[mat.end()..], mat.as_str().trim());
    } else {
        return (input, "");
    }
}

/// Match an ID token.
///
/// An ID token starts with an underscore or letter and
/// contains underscores, letters, or numbers
///
/// # Examples
///
/// ```
/// let (rest, id) = match_id("example_id");
/// assert_eq!(Some(Token::Id("example_id")), id);
/// ```
fn match_id(input: &str) -> (&str, Option<Token>) {
    let re = Regex::new(r"^[_a-zA-Z][_a-zA-Z0-9]*(\s|$)").unwrap();
    let (rest, mat) = match_re(re, input);
    if RESERVED_WORDS.contains(&mat) {
        return (input, None);
    } else if mat != "" {
        return (rest, Some(Token::Id(mat.to_string())));
    } else {
        return (rest, None);
    }
}

/// Match an Num token.
///
/// An Number token consists of whole numbers 0-9
///
/// # Examples
///
/// ```
/// let (rest, id) = match_num("1234");
/// assert_eq!(Some(Token::Num(1234)), id);
/// ```
fn match_num(input: &str) -> (&str, Option<Token>) {
    let re = Regex::new(r"^[0-9]+(\s|$)").unwrap();
    let (rest, mat) = match_re(re, input);
    if mat != "" {
        return (rest, Some(Token::Num(mat.parse::<i32>().unwrap())));
    } else {
        return (rest, None);
    }
}

/// Match an Real token
fn match_real(input: &str) -> (&str, Option<Token>) {
    let re = Regex::new(r"^[0-9]+\.[0-9]+(\s|$)").unwrap();
    let (rest, mat) = match_re(re, input);
    if mat != "" {
        return (rest, Some(Token::Real(mat.parse::<f32>().unwrap())));
    } else {
        return (rest, None);
    }
}

fn match_boolean(input: &str) -> (&str, Option<Token>) {
    let re = Regex::new(r"^((True)|(False))(\s|$)").unwrap();
    let (rest, mat) = match_re(re, input);
    if mat != "" {
        let b = if mat == "True" { true } else { false };
        return (rest, Some(Token::Boolean(b)));
    } else {
        return (rest, None);
    }
}

fn match_static_string(input: &str) -> (&str, Option<Token>) {
    let re = Regex::new(r#"^"[[:ascii:]]+"(\s|$)"#).unwrap();
    let (rest, mat) = match_re(re, input);
    if mat != "" {
        return (
            rest,
            Some(Token::StaticString(
                mat.strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap()
                    .to_owned(),
            )),
        );
    } else {
        return (rest, None);
    }
}

fn match_symbol<'a>(input: &'a str, symbol: &'a str, token: Token) -> (&'a str, Option<Token>) {
    let mut re_str = "^".to_owned();
    re_str.push_str(symbol);
    let re = Regex::from_str(&re_str).unwrap();
    let (rest, mat) = match_re(re, input);
    if mat != "" {
        return (rest, Some(token));
    } else {
        return (rest, None);
    }
}

fn match_comma(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, ",", Token::Comma);
}

fn match_colon(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, ":", Token::Colon);
}

fn match_semicolon(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, ";", Token::Semicolon);
}

fn match_lparen(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\(", Token::Lparen);
}

fn match_rparen(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\)", Token::Rparen);
}

fn match_lbracket(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\[", Token::Lbracket);
}

fn match_rbracket(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\]", Token::Rbracket);
}

fn match_lbrace(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\{", Token::Lbrace);
}

fn match_rbrace(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\}", Token::Rbrace);
}

fn match_period(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\.", Token::Period);
}

fn match_plus(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\+", Token::Plus);
}

fn match_minus(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\-", Token::Minus);
}

fn match_star(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\*", Token::Star);
}

fn match_forwardslash(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, "/", Token::ForwardSlash);
}

fn match_equal(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, "=", Token::Equal);
}

fn match_lessthan(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, "<", Token::LessThan);
}

fn match_greaterthan(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, ">", Token::GreaterThan);
}

fn match_ampersand(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\&", Token::Ampersand);
}

fn match_bar(input: &str) -> (&str, Option<Token>) {
    return match_symbol(input, r"\|", Token::Bar);
}

fn match_reserved_word<'a>(
    input: &'a str,
    word: &'a str,
    token: Token,
) -> (&'a str, Option<Token>) {
    let mut re_str = "^".to_owned();
    let re_str_suffix = r"(\s|$)";
    re_str.push_str(word);
    re_str.push_str(re_str_suffix);

    let re = Regex::from_str(&re_str).unwrap();
    let (rest, mat) = match_re(re, input);
    if mat != "" {
        return (rest.trim(), Some(token));
    } else {
        return (rest, None);
    }
}

fn match_if(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "if", Token::If);
}

fn match_else(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "else", Token::Else);
}

fn match_elseif(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "elseif", Token::ElseIf);
}

fn match_for(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "for", Token::For);
}

fn match_while(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "while", Token::While);
}

fn match_function(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "fn", Token::Function);
}

fn match_let(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "let", Token::Let);
}

fn match_int(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "int", Token::Int);
}

fn match_bool(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "bool", Token::Bool);
}

fn match_float(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "float", Token::Float);
}

fn match_string(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "string", Token::String);
}

fn match_char(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "char", Token::Char);
}

fn match_mut(input: &str) -> (&str, Option<Token>) {
    return match_reserved_word(input, "mut", Token::Mut);
}

fn match_whitespace(input: &str) -> (&str, Option<Token>) {
    let re = Regex::new(r"^\s").unwrap();
    let (rest, _) = match_re(re, input);
    return (rest, Some(Token::Passthrough));
}

fn match_error(input: &str) -> (&str, Option<Token>) {
    panic!("Invalid token: {}", input);
}

pub fn test_chapter_2() {
    println!("\n----Test Chapter 2----\n");
    let tokens = tokenize("57 if abcd 64.0 True False \"Hello World :)\" , { } [ ] . / + - * = > < | & if else elseif for while fn let int bool float string char mut");
    println!("{:?}", tokens);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_id() {
        assert_eq!(match_id("a"), ("", Some(Token::Id("a".to_string()))));
        assert_eq!(
            match_id("_TEST"),
            ("", Some(Token::Id("_TEST".to_string())))
        );
        assert_eq!(
            match_id("abcd1234 qwer7890"),
            ("qwer7890", Some(Token::Id("abcd1234".to_string())))
        );
        assert_eq!(match_id("~"), ("~", None));
        assert_eq!(match_id("a"), ("", Some(Token::Id("a".to_string()))));
        assert_eq!(match_id("1234abcd a"), ("1234abcd a", None));
        assert_eq!(match_id("if"), ("if", None));
    }

    #[test]
    fn test_match_num() {
        assert_eq!(match_num("57"), ("", Some(Token::Num(57))));
        assert_eq!(match_num("64 64"), ("64", Some(Token::Num(64))));
        assert_eq!(match_num("_32"), ("_32", None));
    }

    #[test]
    fn test_match_real() {
        assert_eq!(match_real("57.0"), ("", Some(Token::Real(57.0))));
        assert_eq!(match_real("64.0 64.0"), ("64.0", Some(Token::Real(64.0))));
        assert_eq!(match_real("32"), ("32", None));
    }

    #[test]
    fn test_match_boolean() {
        assert_eq!(match_boolean("True"), ("", Some(Token::Boolean(true))));
        assert_eq!(
            match_boolean("False 64"),
            ("64", Some(Token::Boolean(false)))
        );
        assert_eq!(match_boolean("TrueFalse"), ("TrueFalse", None));
    }

    #[test]
    fn test_match_static_string() {
        assert_eq!(
            match_static_string("\"Hello World\""),
            ("", Some(Token::StaticString("Hello World".to_string())))
        );
        assert_eq!(
            match_static_string("\"~\" rest"),
            ("rest", Some(Token::StaticString("~".to_string())))
        );
        assert_eq!(match_static_string("TrueFalse"), ("TrueFalse", None));
    }

    #[test]
    fn test_match_comma() {
        assert_eq!(match_comma(","), ("", Some(Token::Comma)));
        assert_eq!(match_comma(",89"), ("89", Some(Token::Comma)));
    }

    #[test]
    fn test_match_colon() {
        assert_eq!(match_colon(":"), ("", Some(Token::Colon)));
        assert_eq!(match_colon(":89"), ("89", Some(Token::Colon)));
    }

    #[test]
    fn test_match_if() {
        assert_eq!(match_if("if ()"), ("()", Some(Token::If)));
        assert_eq!(match_if("if89"), ("if89", None));
    }
}
