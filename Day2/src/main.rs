use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let contents = fs::read_to_string(filename).expect("Uh Oh");

    let v: Vec<&str> = contents.lines().collect();

    part1(&v);
    part2(&v);

}

fn part1(input: &Vec<&str>) -> u32 {
   let mut x = 0;
   let mut y = 0;

   for command in input.iter() {
       let (direction, value) = command.trim().split_once(' ').unwrap();
       let value_num = value.parse::<u32>().unwrap();
        match direction {
           "down" => y = y + value_num,
           "up" => y = y - value_num,
           "forward" => x = x + value_num,
           _ => ()
       }
   }
   println!("{}", x*y);

   return x*y;
}

fn part2(input: &Vec<&str>) -> u32 {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
 
    for command in input.iter() {
        let (direction, value) = command.trim().split_once(' ').unwrap();
        let value_num = value.parse::<u32>().unwrap();
         match direction {
            "down" => aim = aim + value_num,
            "up" => aim = aim - value_num,
            "forward" => {x = x + value_num; y = y + (aim * value_num)}
            _ => ()
        }
    }
    println!("{} {}", x, y);
    println!("{}", x*y);
 
    return x*y;
 }


#[cfg(test)]
mod tests {
    const INPUT: &'static str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";
    
        use super::{part1, part2};
    
        #[test]
        fn solve_day_1() {
            let input = INPUT.lines().collect();
            assert_eq!(part1(&input), 150);
            assert_eq!(part2(&input), 900);
        }
}