use std::io;

fn main() {
    //println!("Hello, world!");
    println!("Guess-Who-s-Back-π§π€¨π€π­π­π­");
    println!("Input your guessss");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read lineπ«ππ£");
    println!("You guessed: {guess}");
    
}
