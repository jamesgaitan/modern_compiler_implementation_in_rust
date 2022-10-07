extern crate lalrpop;

mod ast;
mod parser;

pub fn test_chapter_3() {
    println!("\n----Test Chapter 3----\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_float() {
        let src = "3.14";
        let should_be = ast::Atom::Float(3.14);

        let got = parser::AtomParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_an_identifier() {
        let src = "x";
        let should_be = ast::Atom::Id(String::from(src));

        let got = parser::AtomParser::new().parse(src).unwrap();
        assert_eq!(got, should_be);
    }
}
