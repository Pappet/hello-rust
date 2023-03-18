use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let x1 = 5;
    let x2 = 90;
    let x = x1 + x2 + x1;
    let pi = 3.141592;
    let decimal = 4;

    println!("{pi:.decimal$}");
    //println!("x1 is {x1} and x2 is {x2} and the result is {x}");
    //println!("{number:0>width$}", number = 1, width = 5);
    //println!("My name is {0}, {1} {0}", "Bond", "James");

    let stdout = stdout();
    let message: String = format!("x1 is {0} and x2 is {1} and the result is {2}", x1, x2, x);
    let message2 = format!(
        "{zahl1}+{zahl2}+{zahl1}={ergebnis}",
        zahl1 = x1,
        zahl2 = x2,
        ergebnis = x
    );
    //let message = String::from("Is `x` 10 or 100? x = {}");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message2.as_bytes(), width, &mut writer).unwrap();
}
