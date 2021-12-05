use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Uh Oh");

    part1(&contents);
    // part2(&v);
    
}

fn part1(input: &str) {
    let rows = input.lines().count();

    let (gamma, epislon) = (0..12).map(|i| input.lines().filter(|line| line.trim().as_bytes()[i] == b'1').count()).fold((0,0), |(g,e), ones| {
        // ones is the count of items with `1` as the most sig bit

        // if the count of ones is greater than half then "increment" gamma by bit shifting left and "adding" one (bitwise or) and bitshifting episilon right left
        // else the count is less then bitshift gammer one left and add one to episilon
        
        let new_g = g << 1;
        let new_e = e << 1;

        if ones * 2 > rows {
            (new_g | 1, new_e)
        } else {
            (new_g, new_e | 1)
        }
    });
    println!("{} {}", gamma, epislon);
    println!("{}", gamma * epislon)
}

// fn part2(input: &Vec<&str>) -> u32 {
//     let mut x = 0;
//     let mut y = 0;
//     let mut aim = 0;
 
//     for command in input.iter() {
//         let (direction, value) = command.trim().split_once(' ').unwrap();
//         let value_num = value.parse::<u32>().unwrap();
//          match direction {
//             "down" => aim = aim + value_num,
//             "up" => aim = aim - value_num,
//             "forward" => {x = x + value_num; y = y + (aim * value_num)}
//             _ => ()
//         }
//     }
//     println!("{} {}", x, y);
//     println!("{}", x*y);
 
//     return x*y;
//  }


#[cfg(test)]
mod tests {
    const INPUT: &'static str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";
    
        use super::{part1, /*part2*/};
    
        #[test]
        fn solve_day_1() {
            let input = INPUT;
            assert_eq!(part1(input), 150);
            // assert_eq!(part2(&input), 900);
        }
}