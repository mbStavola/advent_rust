mod advent_helper;
use advent_helper::open_problem_input;

fn main() {
    let input = open_problem_input("../../problems/problem_one.txt");

    let mut found = false;
    let floor = input.chars()
        .enumerate()
        .fold(0,
            |acc, c| {
                let movement = match c {
                    (_, '(') => 1,
                    (_, ')') => -1,
                    _ => 0
                };

                if !found && acc == -1 {
                    found = true;
                    let (i, _) = c;
                    println!("Position {} puts Santa in the basement!", i);
                }

                acc + movement
            }
        );

    println!("Santa is at floor #{}", floor);
}
