// fn main() {
//     let string1 = String::from("abcd");
//     let string2 = "xyz";

//     let result = longest(&string1, &string2.into());
//     println!("The longest string is {}", result);
//     println!("{}", string1);
// }

// fn longest(x: &String, y: &String) -> String {
//     if x.len() > y.len() {
//         x.into()
//     } else {
//         y.into()
//     }
// }

//This one executed successfully as they are all owned types

// //lets try this 
fn main() {
   
    let string2 =  String::from("xyz");
    let result;
    
    let string1 = String::from("abcd");
    result = longest(&string1, &string2);

    // check(string1);// it will give compile error
    
    println!("The longest string is {}", result);
    check(string1);

}

// fn longest<'a>(x: &String, y: &'a String) -> &'a String { this will give error
// fn longest<'a>(x: &'a String, y: &'a String) -> &'a String {// this lifetime specifies scope relation between parameters and output
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
//the smaller lifetime within parameters as taken lifetime for output lifetime

fn longest<'a>(x: &'a String, y: & String) -> &'a String {// this lifetime specifies scope relation between parameters and output
    x //here we dont need to specify lifetime specification for y and
}


fn check(x:String) -> bool {
    true
}

//--------------------life time elision rules------------------------
            // 1.Each parameteer reference will get its own lifetime
            // 2.If there is only parameter then its lifetime will be assigned to all output parameters
            // 3.If there any self paramater then its lifetime will be assigned to all output parameters

//--------------------Static liftimes--------------------------------------------these will be have lifetime of entire duration of program