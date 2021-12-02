use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let v: Vec<u16> = contents.split('\n').collect::<Vec<&str>>().iter().map(|s| s.parse().unwrap()).collect();

    let mut count = 0;
    let mut last = 0;
    let iter = v.windows(3);

    for group in iter {
        println!("{:?}", group);
        let curr_depth: u16 = group.iter().sum();
        
        if last != 0 && curr_depth > last {
            println!("{} is greater than {}", curr_depth, last);
            count = count + 1;
        }
        last = curr_depth;
    }
    println!("{}", count);
}