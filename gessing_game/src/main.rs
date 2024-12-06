use std::io;
use rand::Rng;

fn main() {
    println!("*** Guess the number game ***");

    let secret_number = rand::thread_rng().gen_range(1..51);
    let mut count:u32 = 10;

    loop {
        if(count == 0){
            println!("YOU LOSE!");
            break;
        }

        println!("INPUT YOUR GUESS");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
                .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess == secret_number {
            println!("YOU WIN!");
            break;
        }

        if(guess > secret_number) {
            println!("Try another Lower!");
            count -= 1;
            if(count > 0) {
                println!("You have {} turn left! ", count);
            }
            continue;
        }

        if guess < secret_number {
            println!("Try another Upper!");
            count -= 1;
            if count > 0 {
                println!("You have {} turn left! ", count);
            }
            continue;
        }
    }
    println!("*** Thank you! ***");
}
