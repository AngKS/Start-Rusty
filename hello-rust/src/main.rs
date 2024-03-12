use ferris_says::say;
use std::io::{stdout, BufWriter};


fn main() {
    let stdout = stdout();
    let message: String = String::from("Hello Rusties!");
    let width: usize = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    // Trying out formatted print strings as well
    let result = 2 + 2;

    println!("2 + 2 = {}", result);
    println!("Formatting the result in Hex form would be: {:X}", result);
    println!("Formatting the result in Binary form would be: {:b}", result);



}
