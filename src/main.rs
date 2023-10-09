use std::io;
use rand::Rng;
use std::cmp:Ordering;
fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..100); 
    println!("Secret number is {}", secret_number);

    println!("Please input yours guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type the number!")
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small!");
        Ordering::greater => println!("too large!");
        Ordering::equal => println!("you win!");
    }
    println!("Your guessed: {}", guess);
}