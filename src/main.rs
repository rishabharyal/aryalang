use aryalang::Aryalang;

fn main() {
    let mut al = Aryalang::new(
        r#"
if (10 == 100) {
let xxx = 10;	
}
    let n = 10.5*5+43(99/11);
    let x = 55;
let y = "Hello World";

if (n <= 10) {
    let x = 55;
    out = 7;
}
"#
        .to_string(),
    );
    al.run();
}
