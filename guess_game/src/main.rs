use std::io;

fn main() {
    println!("Guess a number");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    print!("You guessed: {guess}");
}
