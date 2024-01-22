use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){
    
    let secret_num = rand::thread_rng().gen_range(1..=100);
    let mut guesses = 0;
    
    loop{
        let mut guess = String::new();      // has to be looped 
        println!("Guess the number");

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        guesses+=1;
        
        let guess: u32 = match guess.trim().parse() {       // *match 
            Ok(num) => num,
            Err(_) =>{
                guesses-=1;
                continue;
            },
        };

        println!("You guessed {guess}");
        
        match guess.cmp(&secret_num){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!!!");
                println!("You took {} guesses", guesses);
                break;
            },
        }
    }   
}