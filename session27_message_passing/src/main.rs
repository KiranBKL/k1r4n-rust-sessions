use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // let (tx, rx) = mpsc::channel();//mpsc multiple producers single consumer

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();//here this thread taken ownership of tx
    //     // println!("{}",val);//it will give u error as .send has taken ownereship
    // });

    // let received = rx.recv().unwrap();//receiver
    // println!("Got: {}", received);

    // Sending Multiple Values and Seeing the Receiver Waiting
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //lets create another producer but we will get them in unordered
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}