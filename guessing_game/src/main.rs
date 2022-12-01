use std::io;
use rand::Rng;
fn main() {
    const N: usize = 20;
    println!("Guess the number! Enter a value from 0 to {N}");
    loop {
        let random = rand::thread_rng().gen_range(0..=N);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = match input.trim().parse::<usize>() {
            Ok(n) => {
                if n > N {
                    println!("Please enter a value between 0 and {N}");
                    continue;
                }
                n
            },
            Err(e) => {
                println!("{e}");
                println!("Please enter a value between 0 and {N}");
                continue;
            }
        };
        println!("You've entered {input}");
        println!("Random number was {random}");
        let cmp = input.cmp(&random);
        if cmp.is_eq() {
            println!("You win!");
            println!("Press any other key to exit");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line!");
            println!("Exiting...");
            std::process::exit(0);
        }  else {
            println!("Try again!");
        }

       
   
    }
}
