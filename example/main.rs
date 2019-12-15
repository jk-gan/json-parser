use json_parser::parser::Parser;

fn main() {
    let json = String::from("{a: 1}");
    let parser = Parser::new(json);
    let result = parser.parse_object();
}
