use aryalang::Aryalang;
use std::fs;

fn main() {

    // expect a parameter which will be a file name and read the contents of the file and pass it to the Aryalang::new() function
    
    // read a file and get its contents
    let contents = fs::read_to_string("main.al").expect("Something went wrong reading the file");

    let mut al = Aryalang::new(
        contents
    );

    al.run();
}
