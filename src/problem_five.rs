mod advent_helper;
use advent_helper::open_problem_input;

extern crate regex;
use regex::Regex;

fn main() {
    let input = open_problem_input("../../problems/problem_five.txt");

    let rule_one = Regex::new("[aeiou]").unwrap();
    let rule_two = Regex::new(r"([a-z])\1").unwrap();
    let rule_three = Regex::new("(ab|cd|pq|xy)").unwrap();

    let nice_words = input.split("\n")
        .fold(0,
            |acc, word| {
                let nice_tuple = (
                    rule_one.is_match(word),
                    rule_two.is_match(word),
                    !rule_three.is_match(word)
                );

                if let (true, true, true) = nice_tuple {
                    acc + 1
                } else {
                    acc
                }
            }
        );

    println!("Number of nice words: {}", nice_words);
}
