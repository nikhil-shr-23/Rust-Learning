use std::io;
// using standard input output library


fn main(){
   println!("Guess the number please");

   println!("enter your guess here");

   let mut nik = String::new();
   // creating new instance of string named nik


   io::stdin()
     .read_line(&mut nik)
     .expect("failed to read lines");


    println!("You guessed: {nik}");
}

//somehow this is working 

//to learn - std and input output using rust 