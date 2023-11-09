use std::io;

//lets implement own Ordering cmp 
enum KiransOrdering {
    AIsLesser,
    AIsGreater,
    Equal,
}

fn compare(a:u32,b:u32) -> KiransOrdering {
    if a>b {
        KiransOrdering::AIsGreater
    }
    else if a<b {
        KiransOrdering::AIsLesser
    }
    else {
        KiransOrdering::Equal
    }
}
fn main() {
    println!("Hello, world!");
    let mut a=String::new();
    let mut b=String::new();
    io::stdin().read_line(&mut a).expect("read line failed");
    io::stdin().read_line(&mut b).expect("readLine failed");
    let a:u32=a.trim().parse().expect("number has been expected");
    let b:u32=b.trim().parse().expect("number has been expected");

    match compare(a, b) {
        KiransOrdering::AIsGreater => {
            println!("a is greater {a},{b}")
        }
        KiransOrdering::AIsLesser => {
            println!("b is greater {a},{b}")
        }
        KiransOrdering::Equal => {
            println!("both r equal {a},{b}")
        }
    }
    //------------------lets talk about option enum--------------------
    //option enum used to get rid of the which is not there a compile time only 
    enum Option<T> {
        None,
        Some(T),
    }  //here T is generic we will talk about that in coming sessions
    //it is deaulty avail in prelude 
}
