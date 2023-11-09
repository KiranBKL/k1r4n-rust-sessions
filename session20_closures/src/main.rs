use std::time::Duration;
use std::thread;

struct Cacher<T> 
    where T: Fn(u32) -> u32
{
    calculation:T,
    value : Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
    {
        fn new(calculation:T) -> Cacher<T> {
            Cacher {
                calculation:calculation,
                value:None
            }
        }

        fn value(&mut self,arg:u32) -> u32 {
            match self.value {
              Some(value) => value,
              None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
              }
            }
        }
    }
fn main() {
    let intensity = 10;
    let random_number = 7;
    generate_workout(intensity, random_number);

    //closures can use same scope variables

    let x = 4;
    let equal_to_x = |y| y==x;
    assert!(equal_to_x(4));

    //closures are three types:
        // 1.Fn      :borrows values
        // 2.FnOnce  :move will be taken
        // 3.FnMut   :can change the environment since it mutably borrows val-use.

    //1. above one is Fn type

    //2.this is second types
    // let x = vec![1, 2, 3];
    // let equal_to_x = move |z| z == x;
    // println!("can't use x here: {:?}", x);
    // let y = vec![1, 2, 3];
    // assert!(equal_to_x(y));

    // let x = 5;
    // let consume_x = move || x; // FnOnce closure
    // let result = consume_x();
    // // Attempting to call it again would result in a compilation error.
    // // let result2 = consume_x(); // Error: use of moved value
    // println!("Result: {}", result);

    //3.this is the third step
    let mut x = 5;
    let mut modify_x = |y| {
        x += y;
        x
    }; // FnMut closure
    let result = modify_x(3);
    println!("Result: {}", result);
    let result2 = modify_x(2);
    println!("Result2: {}", result2);
       
}

fn generate_workout(intensity: u32, random_number: u32) {
    //here expensive function has been called multiple times unnecessarily we can do like this
    // let mut expensive_result = Cacher::new(expensive_cal);
    //lets implement this using closures
    let mut expensive_result = Cacher::new(|num|{
        println!("Calculating intensity");
        thread::sleep(Duration::from_secs(2));
        num
    });

    //closures can use same scope variables

    if intensity < 25 {
        println!("mate, do {} pushups",expensive_result.value(intensity));
        println!("do {} situps",expensive_result.value(intensity));
    }
    else if random_number == 3 {
            println!("Take a break");//when executed this then we r calling this function unnecessarly ,how can we solve this lets do this......
        }
    else {
        println!("run for {} minutes",expensive_result.value(intensity));
    }

    
}

fn expensive_cal(intensity: u32) -> u32 {
    println!("calculating intensity");
    thread::sleep(Duration::from_secs(3));
    intensity
}

