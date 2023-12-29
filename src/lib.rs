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
        let result = parser.parse().expect("Parse Error: \n");
        if result.len() == 0 {
            print!("{:?}", "No tokens to execute.");
            return
        }

        let mut seman_analyzer = core::Analyzer::new(result);
        let result_boolean = seman_analyzer.parse();

        if result_boolean == false {
            print!("{:?}", "^^ Please see error above.");
            return
        }

        print!("{:?} {:?}", result_boolean, "fyck this sghit.......................");
    }
}
