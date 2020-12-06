extern crate advent_of_code_2019;

use self::advent_of_code_2019::*;

fn main() {
    let input = get_input("day_1.txt");

    let total_weight: u32 = input.split_whitespace().map(|mass| {
        get_module_fuel(mass.parse().unwrap()) as u32
    }).sum();

    println!("{}", total_weight);
}

fn get_module_fuel(mass: i32) -> i32 {
    let fuel = (mass as f32 / 3.0).floor() as i32 - 2;

    if fuel <= 0 {
        0
    } else {
        fuel + get_module_fuel(fuel)
    }
}
