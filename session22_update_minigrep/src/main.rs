use std::env;
use std::process;
use session22_update_minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // // let query = &args[1];
    // // let file_path = &args[2];

    // // let Config {query, file_path}= parse_config(&args);//this struct destructuring
    // let config = Config::build(&args);
    // let config = match config {
    //     Ok(config) => config,
    //     Err(e) => {
    //         eprintln!("Error: {}", e);
    //         process::exit(1);
    //     }
    // };

    // we can do like this also
    // let config = Config::build(&args).unwrap_or_else(|err| {    // let config = Config::build(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
    //     println!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //we have to handle when number of arguments does not match

    //reading a file

    
    if let Err(e) = session22_update_minigrep::run(config) {//explicit import
        println!("Application error: {e}");
        process::exit(1);
    }
//    let content = fs::read_to_string(config.file_path)
//        .expect("file reading failed");

  //  println!("{}", content);
    //lets get the query and path from parser
}
//IGNORE_CASE=1 cargo run -- To kepler452.txt we can use this for ignore case
//

//