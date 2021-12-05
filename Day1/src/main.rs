use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filename = &args[1];

    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Uh Oh");

    let v: Vec<u16> = contents.lines().collect::<Vec<&str>>().iter().map(|s| s.parse().unwrap()).collect();

    part1(&v);
    part2(&v);
    
}

fn part1(input: &Vec<u16>) {
    let mut count = 0;
    let mut last = 0;
    for depth in input.iter() {
        let curr_depth = depth.clone();
        
        if last != 0 && curr_depth > last {
            println!("{} is greater than {}", curr_depth, last);
            count = count + 1;
        }
        last = curr_depth;
    }
    println!("{}", count);
}

fn part2(input: &Vec<u16>) {
    let mut count = 0;
    let mut last = 0;
    let iter = input.windows(3);

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