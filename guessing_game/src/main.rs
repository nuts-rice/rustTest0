use std :: io;
//Like Result, Ordering is another enum, but the 
//variants for Ordering are Less, Greater, and Equal.
use std :: cmp :: Ordering;
use rand::Rng;



fn main() 
{
    println!("Guess the number!");
    //let random gen number be rng number inbetween range of 1-100
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);
    //wheeeeee looping!!!!
    //***Remember to always indent correctly!!!***
    loop
    {
        println!("Please input a number");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //let guess be unsigned 32 bit number type 
        //convert guess to u32 type 
        //The parse method on strings parses a string into some kind of number.
        let guess: u32 = match guess.trim().parse()
        {
            /* Switching from an expect call to a match expression is how you 
            generally move from crashing on an error to handling the error */
            Ok(num) => num,
            Err(_) => continue,
        
        };
        println!("You guessed {}", guess);
        //cmp method compares 2 values
        //returns varient of Ordering enum
        match guess.cmp(&secret_number)
        //use match expression to decide what to do next based on
        //which varient of ordering is returned
        {

        /*  A match expression is made up of arms. An arm consists of a pattern and the 
            code that should be run if the value given 
            to the beginning of the match expression fits that arm’s pattern.  */

            Ordering :: Less => println!("too small"),
            Ordering :: Greater => println!("too big"),
            Ordering :: Equal => 
            {
               /*  Adding the break line after You win! makes the program 
                exit the loop when the user guesses the secret number correctly.  */
                println!("Guess is right");
                break;
            }

        }
    }

}

