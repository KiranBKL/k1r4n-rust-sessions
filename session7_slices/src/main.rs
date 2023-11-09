//Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
fn main() {
    let mut s = String::from("kiran kumar");
    let k = &s[0..5];
    let k2 = &s[6..11];
    println!("{k},{k2}");
    println!("{s}");

    let word = first_word(&s);//this taken mutable refeerence
    
   // s.clear();
    println!("the first word is: {}", word);//that references has scope up to this but in above line mutable reference has been used it will throw error
    s.clear();
    //&slice[starting_index..ending_index]

    //string slicing also works on string literals

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);

    //we can apply same in arrays
    //&String and &str can be passed to argument &str but &str cant be passed to argument &String i only accepts &Strin

}
fn first_word(s: &str) -> &str { //fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}