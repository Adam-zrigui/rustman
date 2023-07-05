use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main() {
    print!("Enter a random number");
    loop{
        let mut guess = String::new();
        let sec = rand::thread_rng().gen_range(0..100);

        io::stdin().read_line(&mut guess).expect("Error reading");
        let guess: u32 =  match  guess.trim().parse() {
Ok(num ) => num,
            Err(_) => continue
        };
        println!("{}", guess);
        match guess.cmp(&sec) {
            Ordering::Less => println!( "{}", "too small".red()),
            Ordering::Greater => println!("{}","too much".red()),
            Ordering::Equal => {println!("{}","u got it".green()); break}
        }
  }
}
