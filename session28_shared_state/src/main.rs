//we can do share state using mutex

use std::sync::{Mutex,Arc};
use std::thread;

fn main() {
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);

    //lets share mutex data between multiple threads

    // let counter = Mutex::new(0);
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();//here error will occur as counter has been moved here as it has only one owner lets do it uding Rc so Rc also wont work 
            //there is one way we can do this is : Arc (Atomic Reference Counter)


            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}