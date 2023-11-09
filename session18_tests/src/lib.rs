//assert
//assert_eq!
//assert_ne!

//cargo test
//cargo test it_works
//cargo test can

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

pub fn add_two(x:u32) -> u32 {
    x+2
}

#[cfg(test)]
mod tests {

    use super::*;//bringing parent functions into scope

    #[test] //this attribute indicates function as test function
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 6,
            height: 9
        };

        let smaller = Rectangle {
            width: 3,
            height: 6
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    #[ignore]
    fn smaller_can_hold_large() {
        let larger = Rectangle {
            width: 6,
            height: 9
        };

        let smaller = Rectangle {
            width: 3,
            height: 6
        };
        assert!(smaller.can_hold(&larger));
    }

    //adding custom failure message
    #[test]
    #[ignore]
    fn smaller_can_hold_large_message() {
        let larger = Rectangle {
            width: 6,
            height: 9
        };

        let smaller = Rectangle {
            width: 3,
            height: 6
        };
        assert_eq!(smaller.can_hold(&larger), true,"larger rectangle can hold  smaller rectangle");//custom message
        //we can do custom mesage for every assert type

        //lets discuss about should_panic
    }

       #[test]
        #[should_panic]
        fn greater_than_100() {
            Guess::new(200);//new will panic and we made argument should_panic this test will be pass
        }

        #[test]
        #[should_panic]
        #[ignore]
        fn greater_than_100_2() {
            Guess::new(90);//this test wont pass as new wont panic
        }

        //we can do using Result Enum
        #[test]
        fn it_works_2() -> Result<(), String> {
            if 2 + 2 == 4 {
                Ok(())
            } else {
                Err(String::from("two plus two does not equal four"))
            }
        }
}

// running 3 tests
// test tests::smaller_can_hold_large ... ignored
// test tests::it_works ... ok
// test tests::larger_can_hold_smaller ... ok

// test result: ok. 2 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

//    Doc-tests session18_tests  //documenting purpose

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//by default every test will get one thread to run but we can change that to make it sequential by by running  "cargo test -- --test-threads=1"

// when test failed it wont show success ones but we can view output using command "cargo test -- --show-output"


