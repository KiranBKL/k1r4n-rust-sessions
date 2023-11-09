use std::io;
// scalar types:
//     1.Integers--signed(i8-i128) ans unsigned(u8-u128)
//     2.Floating Points--f32 and f64
//     3.Booleans and
//     4.Characters

//     if integres overflowed means u8 last values is 255 if we given 256 
//         1.if we run in debug mod the it will Panic
//         2. in --release flag mode it will assign to (256=0,257=1..n%256)

//     Integer Literals:
//         1.Decimal  9999_99
//         2.Hex      0xfa
//         3.Octal    0o75
//         4.Binary   0b1111_0000
//         5.Byte(u8) b'A'
//     Floating Point-Literals:
//         1.f32
//         2.f64
//         3.e
// //type casting to Int or Float
// //.parse has been used for this
// Compund types:(fixed in size)
//     1.Tuples
//     2.Arrays

fn main() {
//    let x="42".parse().expect("not a number"); it gives error because when we are parsing then we have to decalre parsing type
   let x:u8="18".parse().expect("not a nmber");
   let x:f32="18".parse().expect("not a nmber");
   println!("{x}");
   let x:f32=18.0;
   println!("{x}");
   let x:f32=18.1;
   println!("{x}");
   //https://doc.rust-lang.org/stable/std/fmt/index.html to study std:fmt

   let x=8.0;
   println!("{x}");

   let x= 8;
   println!("{}",x);
    //https://stackoverflow.com/questions/21747136/how-do-i-print-in-rust-the-type-of-a-variable
   //default floting type is f64
   //default numeric type is i32
   //and there is no default values in rust

   let t = true;
   let f: bool = false; // with explicit type annotation
   println!("{t},{f}");

   let c = 'z';
   let z: char = 'ℤ'; // with explicit type annotation
   let cat = '😻';
   println!("{c},{z},{cat}");

   //----------------------Tuples--------------------------
   let data = (1,"g4ng4k1r4n",22);
   let (id,name,age) = data;//Destructuring
   println!("{id} {name} {age}");
   let id= data.0;
   let name = data.1;
   let age = data.2;
   println!("{id} {name} {age}");
   //tuple values can be accessed using .index

   let data = (id,name,age);
   // println!("{data}"); //here tuple doesnot implement std:fmt:Display
   println!("{data:?}");

   let a =[1,2,3,4,5];
   println!("{a:?}");

   let a=[1..5];//it will be used in index slicing
   println!("{a:?}");
   println!("{a:?}");
   let a=[1,2,3,4,5,6];

   let b= &a[1..5];//here we have to referencing when we r slicing
   println!("{b:?}");

   let b=a;
   println!("{b:?}");
   println!("{a:?}");

   let k=9;
   let g=k;
   println!("{k},{g}");

   let k=[3;5];
   let i=k[0];//array elements accessed using index

   let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

   let element:i32;
   if index < a.len() {
   // const element:i32 = a[index];we cant use const(do not attem use a non-const in a const)
   element=a[index];
   } else {
         println!("Index is out of range. The array length is {}.", a.len());
         element=0;
   }


    println!("element at index({index}) is {element}")//we gonna handle exception if entered index greater than len in upcoming tutos

}
