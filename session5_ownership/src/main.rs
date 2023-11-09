//Ownership makes rust unique it allow us code more strictly to get avoid of Garbage Collector
// Ownership rules:
//     1.Each value in rust has an Owner
//     2.There can be only one owner at a time
//     3.When the owner goes out of scope, the value will be dropped.
// Here we gonna know about move and copy and scope and Stack and Heap
//     1.Primtive types always makes copy
//     2.other types makes move eg:String
fn main() {
   let x= 9;
   let y= x;
   println!("{x},{y}");
   let i=String::from("Solana");
 //  let j=i;
//    println!("{i},{j}") here it gives error value borrowed here after move scope of i dropped at line 14
    // to get rid of this we can make referrence or clone

    let j=&i;//it just create referrence pointer
    let k=i.clone();//it will do copy in heap

    println!("{i},{j},{k}");
    let l=i;
    let k=l;

    // Whats the difference between &str and String
    // &str simple string literal which is immutable and fixed size and stored in Stack
    // String is Growable or Mutable and dynamic sized stored in Heap
    // String :  i
                //  name     vale                 index       value
                // ptr        --------------------> 0            b
                // len        3                     1            a
                // capacity   3                     2            t
    //here l and k points to same memory in heap so k become owner and l wont be owner anymore
    //but when we referrence the owner wont change and just points to memory
    // but when we cloned it will create exact same duplicate in memorys another value and another owner

    //-------------------lets see with functions-------------------------
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    //but it is not like this with primitive types as they always makes a copy with functions also

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);

    //we will look into more about referencing in next session
}


fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

