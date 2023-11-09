//In rust strings are UTF-8 encoded
// Strings:
//         1.bytes
//         2.char(scalar)
//         3.graphine clusters

use unicode_segmentation::UnicodeSegmentation;
fn main() {
    let str="hello";//String literal
    let s= String::from("hello");
    let mut k=String::new();
    let x="x";
    let y:String ="y".into();
    k.push('y');//here push takes cahr
    println!("{k}");
    k.push('x');
    println!("{k}");
    k.push_str(&y);//push_str always takes the string literal (String refe)
    println!("{k}");
    k.push_str(x);

    // implementation of push_str
    // pub fn push_str(&mut self, string: &str) {
    //     self.vec.extend_from_slice(string.as_bytes())
    // }
    //x.push_str("x"); push_str method does not implemented for string literals as they are references and not mutable and stored in stack  memory

    println!("{k}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
  //  let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
  //  println!("{s3}");
    println!("{s2}");
    // println!("{s1}") here it will give error as s1 moved already

    //we can do another way using format macro we will see about macros in next sessions

    let s3 = format!("{s1}{s2}");
    println!("{s3}");
    println!("{s2}");
    println!("{s1}");//here it did not give error\

    let s1 = "Hello, world!";
    // let s1 = String::from("hello");
    // let h = s1[0];not implement like this as it stored in heap space and UTF8 encoding
    // some languages will take single byte for char some will take 2 or more bytes

    //iterat/

    let h=s1.chars().nth(0).expect("char is not there");//we can explicitly match or unwrap or can include expect statement
    println!("{h} ");

    let hindi  = String::from("नमस्ते");

    for b in hindi.as_bytes() {
        print!("{b} ");
    }
    println!();
    
    for c in hindi.chars() {
        print!("{c} ");
    }  
    println!();

    //for grapheme clusters we have to add unicode-segmentation crate
    for g in hindi.graphemes(true) {
        print!("{g} ");
    }

    let thirdbyte = hindi.as_bytes()[2];
    let thirdchar = hindi.chars().nth(2).expect("msg");
    let thirdgrapheme = hindi.graphemes(true).nth(2).expect("msg");
    // let slice = &hindi[0..1];//here slicing for 1 char wont work as It implemented for UTF8 encoding

    // println!("{slice} ");
    println!("{thirdbyte}");
    println!("{thirdchar}");
    println!("{thirdgrapheme}")

    //we already discussed about slicing

}
