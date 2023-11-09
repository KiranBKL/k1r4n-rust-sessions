use std::collections::HashMap;
fn main() {
    //creating
    let mut scores = HashMap::new();

    //inserting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //accessing
    let score = scores.get("Blue").copied().unwrap_or(0);//till now we haven't seen unwrap_or ,if unwrap fials it will assign default value 

    //iterating through
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //updating
    scores.insert(String::from("Blue"), 100);
    scores.insert(String::from("Yellow"), 200);


    //deleting
    scores.remove(&String::from("Purple"));// it wont give error if key was not found there also
    scores.remove(&String::from("Yellow"));

    //iterating through
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //Adding a Key and Value Only If a Key Isn’t Present
    scores.entry(String::from("Yellow")).or_insert(5000);
    scores.entry(String::from("Blue")).or_insert(5000);
    scores.entry(String::from("Purple")).or_insert(5000);
    scores.entry(String::from("Orange")).or_insert(5000);

    println!();

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //word count in senetence

    let sentence = String::from("Hello world wonderfull world");
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let mut word_count:HashMap<String,u8> = HashMap::new();
    
    for word in words {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;//deference and update count
    }
    for (key, value) in &word_count {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);// insert will take ownership so we to make a copy rather than move

    println!("{field_name},{field_value}");
}
