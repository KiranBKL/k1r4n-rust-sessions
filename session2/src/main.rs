
#[warn(unused_variables)]
fn main() {
    let x= 6;
    //x=7; this gives error as in rust variables are immutable by default and to make it mutable we have to declare as mut
    println!("x values is {x}");

    let mut y=9;
    //here above line gives warning regarding y has not used before assigning to another value so we gonna use this
    //as we  used #[warn(unused_variables)]
    println!("{}",y);
    y=12;
    //here variable decalred as mut so it wont give error
    println!("y value si {y}");

    //-----------------------Shadowing--------------------
    let y = String::from("Kiran");
    println!("y is {y}");
    //here second y shadows the first y and first y no more in scope

    // const rules:
    //     1.const variable name must be UPPERCASE_SNAKE_CASE
    //     2.These are immutable by default
    //     3.these must be decalred with its type

    //const k=9; it should be declared with type

    const k:u8=9;//1.scope starts here
    //it gives warning regarding UPPERCASE_SNAKE_CASE

    //const k:u16=12; const variales cant be shadowed within scope
    {
        const k:i32=12;//2.scope starts here
        println!("{k}");//2.ends here
    }
    println!("{k}");
    //lets make UPPERCASE_SNAKE_CASE
    const K:i32=15;
    println!("{K}");

    //another example for shadowing
    let s="    ";//type is &str
    let s=s.len();//type is usize and it was shadowed first s

    //mut keyword wont change type like shadowing 
    // let mut s="     ";
    // s=s.len();


//1.ends here
}
