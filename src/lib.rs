pub mod parser;

#[cfg(test)]
mod tests {
    use crate::parser::Parser;

    #[test]
    fn it_works() {
        let json = String::from("{a: 1}");
        let parser = Parser::new(json);
        let result = parser.parse_object();
        assert_eq!(2 + 2, 4);
    }
}
