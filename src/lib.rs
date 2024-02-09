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
        let result_option = seman_analyzer.parse(); // returns Result<bool, error>
        match result_option {
            Ok(_value) => {
            },
            Err(_e) => {
                // Handle the error case, e.g., log the error, return from function, etc.
                // You can use _e to access the error if needed
                println!("Error: {:?}", _e);
            },
        }


    }
}
