use std::thread;
use std::time::Duration;
//threads run simultaneously
// 1.spawn
// 2.join method
// 3/Move
fn main() {
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);//it will trhow error as v borrowed here
    // });
    let handle = thread::spawn(move || {//it takes ownership
        println!("Here's a vector: {:?}", v);
    });
    // println!("{:?}",v); it will give error
    handle.join().unwrap();
}