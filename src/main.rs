use aryalang::Aryalang;

fn main() {
    let mut al = Aryalang::new(
        r#"
if (true) {

let xxx = 10;	
}

    let n = 10*5+43(99/11);

let x = 55;
let y = 33;
let out = 44+2-(81/9);
x+y;
if (n >= 10) {
    x = 22;
    let rishabhj = 33;
}
print(inttostr(out));
"#
        .to_string(),
    );

    al.run();
}
