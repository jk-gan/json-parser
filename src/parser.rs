pub struct Parser {
    json: String,
}

impl Parser {
    pub fn new(json: String) -> Self {
        Parser { json }
    }

    pub fn parse_object(&self) {
        if let Some(x) = self.json.chars().nth(0) {
            if x == '{' {
                // skip_whitespace();

                for char in self.json.chars() {
                    dbg!(char);
                }
            }
        }
    }
}
