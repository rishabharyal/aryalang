struct String {}

impl String {
    pub fn new() -> String {
        String {}
    }

    pub fn is_string(&self, s: &str) -> bool {
        s.starts_with('"') && s.ends_with('"')
    }

    pub fn is_number(&self, s: &str) -> bool {
        s.parse::<f64>().is_ok()
    }

    pub fn is_boolean(&self, s: &str) -> bool {
        s == "true" || s == "false"
    }

    // length
    pub fn length(&self, s: &str) -> usize {
        s.len()
    }

    // to_number
    pub fn to_number(&self, s: &str) -> f64 {
        s.parse::<f64>().unwrap()
    }

    // to_boolean
    pub fn to_boolean(&self, s: &str) -> bool {
        s == "true"
    }
}
