use std::{error, io};
use std::fs;
// we gonna discuss about panic macro,Result Enum and error propogating
use std::{fs::File, f32::consts::E};
use std::io::{ErrorKind, Read};
fn main() {
    //unwinding vs aborting
    let v = vec![1, 2, 3];
  //  v[99];
    //use RUST_BACKTRACE=1 cargo run
    // a();
    // fn a() {
    //     b(2);
    // }

    // fn b(i:i8) {
    //     if i==2 {
    //         panic!("panickdie");
    //     }
    // }

    // let file = File::open("interstellar.txt");
    // let file= match file {
    //     Ok(file) => file,
    //     Err(e) => {
    //         println!("Error opening file: {:?}", e);
    //         return;
    //     }
    // };

    //Handling different type of errors
    let file = File::open("interstellar.txt");
    let file = match file {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("interstellar.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),       
            }
            other_err => panic!("Error opening file: {:?})", other_err)
        } 
    };

    //handling in different ways
    let file = File::open("kepler452b.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("kepler452b.txt").unwrap_or_else(|error| {
                panic!("Error creating file: {:?}", error);
            })
        } else {
            panic!("Problem opening file: {:?}", error);
        }
    });
    read_name_from_file();

    //we can do matching or unwrapping or expect checking 

    // lets discuss about error propogating

    // when we wanna give case of error handling for calling function then we can use error propagation


    
}

fn read_name_from_file() -> Result<(),io::Error>
{
    // let mut file = File::open("kepler452b.txt")?;
    // let mut s = String::new();
    // file.read_to_string(&mut s).expect("error while reading file: ");

    // let mut s = String::new();
    // File::open("kepler452b.txt")?.read_to_string(&mut s).expect("error while reading file: ");

    //we can fs::read_to_string
    fs::read_to_string("kepler452b.txt");

    return Ok(())
}
