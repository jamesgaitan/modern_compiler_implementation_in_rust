extern crate lalrpop;

pub fn test_chapter_3() {
    println!("\n----Test Chapter 3----\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_a_number_atom() {
        let src = "3.14";
        let should_be = Atom::Number(3.14);

        let got = parser::parse_Atom(src).unwrap();
        assert_eq!(got, should_be);
    }

    #[test]
    fn parse_an_identifier() {
        let src = "x";
        let should_be = Atom::Ident(String::from(src));

        let got = parser::parse_Atom(src).unwrap();
        assert_eq!(got, should_be);
    }
}
