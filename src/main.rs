use std::io;

fn main() {
    println!("Guess the number!");

    println!("Nceda ubhale inani oluqashileyo (qash-qash).");

    let mut iqashiso = String::new();

    io::stdin()
        .read_line(&mut iqashiso)
        .expect("Failed to read a line");

    println!("Inani oliqashileyo: {}", iqashiso);
}
