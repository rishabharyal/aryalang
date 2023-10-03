mod core;
pub struct Aryalang {
    input: String,
}

impl Aryalang {
    pub fn new(input: String) -> Self {
        Aryalang { input }
    }

    pub fn run(&mut self) {
        let lexer = core::Lexer::new(&self.input);
        let tokens = lexer.tokenize();
        let mut parser = core::Parser::new(&tokens);
        parser.parse().expect("Parse Error: \n");
    }
}
