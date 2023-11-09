use aryalang::Aryalang;

fn main() {
    let mut al = Aryalang::new(
        r#"
if (10 == 100) {
	print(“Hello, there!”);
}
    let n = 10.5*5+43(99/11);
    let x = 55;
let y = "Hello World";

if (n <= 10) {
    let x = 55;
    out = 7;
}

print(y);
print("The number is: " + out);
    "#
        .to_string(),
    );
    al.run();
}
