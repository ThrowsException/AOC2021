use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Uh Oh");

    let v: Vec<&str> = contents.lines().collect();

    // println!("{:?}", v);

    let mut count = 0;
    let mut last = 0;
    for depth in v.iter() {
        let curr_depth = depth.parse().unwrap();
        
        if last != 0 && curr_depth > last {
            println!("{} is greater than {}", curr_depth, last);
            count = count + 1;
        }
        last = curr_depth;
    }
    println!("{}", count);
}