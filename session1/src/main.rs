use std::io;

// fn main() {
//    println!("Guess the number dude !");
//    let mut guess = String::new();
//    let num = io::stdin().read_line(&mut guess);
//    print!("the guessed num is {guess} and buff num is {num:?}");
// }
// The Result doesn't impl Display, but Results usually impl Debug to see thats why
// to see values it should be implement std::fmt::Display

// fn main()  {
//     println!("Guess the number dude !");
//     let mut guess = String::new();
//     let num = match io::stdin().read_line(&mut guess) {
//         Ok(n) =>{ n}
//         Err(e) =>{
//             eprintln!("Error: {}", e);
//             return;}
//     };
//     print!("the guessed num is {guess} and buff num is {num}");    
//  }
// we handle by match case

// fn main() -> Result<(),io::Error> {
//     println!("Guess the number dude !");
//     let mut guess = String::new();
//     let num = io::stdin().read_line(&mut guess)?;
//     print!("the guessed num is {guess} and buff num is {num}");
//     Ok(())
//  }
// We can use ? to unwrap value from Result type but parent function return Result type

// fn main()  {
//     println!("Guess the number dude !");
//     let mut guess = String::new();
//     let num = io::stdin().read_line(&mut guess).expect("Failed to read line");
//     print!("the guessed num is {guess} and buff num is {num}");
//  }


// fn main() {
//     println!("Guess the number, dude!");
//     let mut guess = String::new();

//     let num = io::stdin()
//         .read_line(&mut guess)
//         .map_err(|e| eprintln!("Failed to read line: {}", e))
//         .expect("Failed to read line");

//     println!("The guessed num is {} and buffer num is {}", guess, num);
// }
// we can use .expect() or .map_err() when we want unwrap and parent function not returning Result type

fn main() -> Result<(),io::Error> {
    println!("Guess the number dude !");
    let mut guess = String::new();
    let num = io::stdin().read_line(&mut guess)?;
    print!("the guessed num is {guess} and buff num is {num}");
    Ok(())
    // we will look guessing number until correct in upcoming sessions
    
 }