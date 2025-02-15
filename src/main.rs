use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let inani_lemfihlakalo = rand::thread_rng().gen_range(1..=100);

    println!("Inani lemfihlakalo ngu: {inani_lemfihlakalo}");

    println!("Bhala iqashiso lakho.");

    let mut iqashiso = String::new();
    
    
    println!("Nceda ubhale inani oluqashileyo (qash-qash).");

    //let mut iqashiso = String::new();

    io::stdin()
        .read_line(&mut iqashiso)
        .expect("Failed to read line");

    println!("Inani oliqashileyo: {}", iqashiso);

    let x = 5;
    let y = 10;
    let z = 8;
    let k = 1;

    println! ("x = {x} and y + 3 = {}", y + 3);
    println! ("z = {z} and k + 2 = {}", k - 10)
}
