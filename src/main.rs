use aryalang::Aryalang;

fn main() {
    let mut al = Aryalang::new(
        r#"
    let n = 10.5;
let y = "Hello World";

function makeit(n) {
    return n + 1;
}

if (n <= 10) {
    out = makeit(n);
}

print(y);
print("The number is: " + out);
    "#
        .to_string(),
    );
    al.run();
}
