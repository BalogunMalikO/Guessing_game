//to import the necessary modules
use std::io;
use rand:: Rng;
use std::cmp::Ordering;


fn main() {
    println!("Hello, Welcome to my gguessing game");

    //This line is used to generate the ranndom secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    

    loop{
       println!("Please input your guess: ");

       //This line save the input of the user as string into guess variable
       let mut guess = String::new();

      //This line accepts input from the user,save it in guess
       io::stdin().read_line(&mut guess).expect("failed to read line");

    // This line converts the guessed input and check if it is a number, if not it ask user to trying the input
       let guess: u32 = match guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };
       println!("you guessed: {guess}");
      
      //This line is use to compare the user input and print if it is higher, lower or equal to the secret number generated 
       match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small!"),
          Ordering::Greater => println!("too big!"),
          Ordering::Equal => {
            println!("You win!");
            break;
         }

        
        }

    }

    
}
