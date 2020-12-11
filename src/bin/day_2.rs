extern crate advent_of_code_2019;

use self::advent_of_code_2019::*;
use self::advent_of_code_2019::int_code::*;
use itertools::Itertools;

fn main() {
    let input = get_input("day_2.txt");

    let code: Vec<i32> = input
        .split(',')
        .map(|op| op.parse().unwrap())
        .collect();

    let mut test_code = code.clone();
    test_code[1] = 12;
    test_code[2] = 2;

    let mut computer = IntCodeComputer::new(test_code);
    computer.run();

    println!("{}", *computer.get(0).unwrap());

    let pairs = (0..99).cartesian_product(0..99);
    let mut answer = 0;
    for pair in pairs {
        let mut try_code = code.clone();
        try_code[1] = pair.0;
        try_code[2] = pair.1;

        let mut computer = IntCodeComputer::new(try_code);
        computer.run();

        if *computer.get(0).unwrap() == 19690720 {
            answer = computer.get(1).unwrap() * 100 + computer.get(2).unwrap();
            break;
        }
    }

    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut computer = IntCodeComputer::new(vec![1, 0, 0, 0, 99]);
        computer.run();
        assert_eq!(2, *computer.get(0).unwrap());
    }

    #[test]
    fn test_multiply() {
        let mut computer = IntCodeComputer::new(vec![2, 3, 0, 3, 99]);
        computer.run();
        assert_eq!(6, *computer.get(3).unwrap());
    }

    #[test]
    fn test_intcode() {
        let mut computer = IntCodeComputer::new(vec![2, 4, 4, 5, 99, 0]);
        computer.run();
        assert_eq!(9801, *computer.get(5).unwrap());

        let mut computer = IntCodeComputer::new(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        computer.run();
        assert_eq!(30, *computer.get(0).unwrap());
        assert_eq!(2, *computer.get(4).unwrap());
    }
}
