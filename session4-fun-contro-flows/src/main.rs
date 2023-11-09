use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let x=
    {
        let y=1;
        y
    };
    println!("{x}");
    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Failed to read line");
    let  k:i32 =k.trim().parse().expect("Integer has been expected");
    let x = even_or_not(k);
    println!("{x}");
    println!("{k}");//All primitive types are Copy we will look into Ownership in next session

    let res = if k%2==0 {true} else {false};//concisized
    println!("{res}");

    //-----------------guessing number unitll corrrect
    // guess_number();

    //returning value from loops
    let mut counter = 0;
    let result = loop {
        let k=1;
        counter += 1;
        if counter == 10 {
            break k;//k is binded tp result
        }
    };
    println!("The result is {result}");

    //-------------------------fibnacci-----------------
    let mut n = String::new();
    println!("how many fibnacci numbers do u want : ?");
    io::stdin().read_line(&mut n).expect("read line fialed");
    let n:u64 = n.trim().parse().expect("Integer has been expected");
    // let mut a = 0;
    // let mut b = 1;
    // let mut i=0;
    // while i<n {
    //     print!("{a},");
    //     let j= a+b;
    //     a = b;
    //     b= j;
    //     i+=1;
    // }

    let fibonacci_series = fibonacci_up_to_n(n);
    // we can iterate through vec using for withou t indexing
    for x in fibonacci_series {
        print!("{x},");
    }

    // println!("{fibonacci_series:?}");//here fibonacci_series gives error as it has been moved in line 53 we will talk about them next session

}

fn fibonacci_up_to_n(n:u64) -> Vec<u64> {

    let mut fib_series = Vec::new();
    let mut a = 0;
    let mut b = 1;
    let mut i=0;
    while i<n {
        fib_series.push(a);
        let j= a+b;
        a = b;
        b= j;
        i+=1;
    }
    fib_series
}

fn even_or_not(k:i32) -> bool {
    if k%2==0 {
        return true;
    }
    else {
    false
    }
}

fn guess_number() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {  //it will iterate untill break
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        // if guess> secret_number {
        //     println!("Too big!");
        // }
        // else if guess<secret_number {
        //     println!("Too small!")
        // }
        // else {
        //     println!("You win!");
        //     break;
        // }

        // Ordering is an Enum
      
        match guess.cmp(&secret_number) {//.cmp returns Ordering Enum type
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }//lets implement withou using .cmp by creating own enum and function in upcoming session
    }
}
