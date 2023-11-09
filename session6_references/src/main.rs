fn main() {
    let s= "kiran";
    let len= cal_len(s);
    println!("{s} len {len}");
    //it wont give error

    let s =String::from("kiran");
    let len =cal_string_len(s);
    // println!("{s} len {len}"); this will give error borrow of moved value as it is not primitive type
    // Polymorphism does not support in rust  so we cant create function fn cal_len(s:&str)

    let mut s =String::from("kiran");
    let len =cal_string_len_2(&s);
    println!("{s} len {len}");

    // we can do like this also
   // let len =cal_len(&s);  
   //---------------------------- mutable reference----------
   change(&mut s);

   let mut s = String::from("hello");

   let r1 = &mut s;
   let r2 = &mut s;

   //println!("{}, {}", r1, r2);//here it will give error as only one mutable reference allowed in one scope

//    let mut s = String::from("hello");
//    let r2 = &mut s;
//    {
//        let r1 = &mut s;
//        //println!("{}, {}", r1, r2);
//    } // here also same in this scope there two mutable references
//    println!("{}", r2);

    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    //     println!("{}", r1);
    // } // here also same in this scope there two mutable references
    // let r2 = &mut s;
    // println!("{}", r2);

    let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // // let r3 = &mut s; // BIG PROBLEM this will give error as mutable and immutable references not allowed

    // println!("{}, {}, and {}", r1, r2, r3);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{}, {}", r1, r2);
    let r3 = &mut s; // BIG PROBLEM this will give error as mutable and immutable references not allowed
    println!("{}", r3);
    //here there is no problem r1 and r2 

    //there is o dangling pointer in rust lets see
    //let reference_to_nothing = dangle();

}
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s//here s goes out of the scope so that reference as to refer nothing but in rust that wont point to anything and its just simply give compile error we will look into this in lifetime session
// }

fn change(s: &mut String) {
    s.push_str(", world");
}

fn cal_len(s:&str) -> usize {
    s.len()
}

fn cal_string_len(s:String) -> usize {
    s.len()
}

fn cal_string_len_2(s:&String) -> usize {
    s.len()
}


// so rules for references are
//     1.rust can be allowed any number of immutable references or one mutable reference 
//     2.References should be vaalid