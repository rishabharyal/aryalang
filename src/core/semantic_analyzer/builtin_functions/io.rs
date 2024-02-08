
struct IO {}

impl IO {
    pub fn read_input() -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
    }

    pub fn print_output(output: String) {
        println!("{}", output);
    }
}
