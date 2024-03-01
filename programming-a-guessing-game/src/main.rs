use std::io;
use rand::Rng;
use std::cmp::Ordering;

//let apples = 5 ; // immutable
//let mut apples = 5 ; // mutable


fn main(){
    
    //let x = 5 ;
    //let y = 10 ;
    
    //println!("x = {x} and y + 2 = {}" , y + 2 );
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}" , secret_number);
   
    loop {
        
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Filed to read line");
    
        let guess : u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }),
        }
    }

}

