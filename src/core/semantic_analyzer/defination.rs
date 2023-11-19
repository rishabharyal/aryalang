pub struct Analyzer {
}

// Scope table
// general variable table just to check if variable exists or not. Maybe not?

#[derive(Debug, Clone)]
pub enum AnalysisError {
    UndefinedVariable { expected: String, found: String },
    // Add more detailed error types if necessary
}

impl Analyzer {
    pub fn new() -> Self {
       Analyzer { }
    }

    pub fn parse(&mut self) -> bool {
        return true;
    }
}
