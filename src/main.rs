use std::io::Read;
use std::fs::File;
use std::collections::HashSet;
use std::fmt;

extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
    problem_one();
    problem_two();
    problem_three();
    problem_four();
}

fn problem_one() {
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

fn problem_two() {
    let input = open_problem_input("../../problems/problem_two.txt");

    let mut wrapping_paper_length = 0;
    let mut ribbon_length = 0;

    for line in input.split("\n") {
        let dimens: Vec<u32> = line.split("x").map(|e| e.parse::<u32>().unwrap()).collect();

        let dimen_max: &u32 = dimens.iter().max().unwrap();
        let ribbon_wrap = 2 * (dimens[0] + dimens[1] + dimens[2] - dimen_max);
        ribbon_length += (dimens[0] * dimens[1] * dimens[2]) + ribbon_wrap;

        //2*l*w + 2*w*h + 2*h*l
        let side_one = dimens[0] * dimens[1];
        let side_two = dimens[1] * dimens[2];
        let side_three = dimens[0] * dimens[2];

        let slack: &u32 = vec![&side_one, &side_two, &side_three].iter().min().unwrap();

        wrapping_paper_length += 2 * (side_one + side_two + side_three) + slack;
    }

    println!("{} sq ft of wrapping paper needed", wrapping_paper_length);
    println!("{} ft of ribbon needed", ribbon_length);
}

fn problem_three() {
    let input = open_problem_input("../../problems/problem_three.txt");

    #[derive(Eq, PartialEq, Hash, Clone, Copy)]
    struct Pair {
        x: i32,
        y: i32
    }

    let mut santa_curr_pos = Pair{ x: 0, y: 0 };
    let mut robo_curr_pos = Pair { x: 0, y: 0 };

    let mut house_set: HashSet<Pair> = HashSet::new();
    house_set.insert(santa_curr_pos);

    for e in input.chars().enumerate() {
        let (index, c) = e;

        let mut pos_ptr: &mut Pair;
        if index % 2 == 0 {
            pos_ptr = &mut robo_curr_pos;
        } else {
            pos_ptr = &mut santa_curr_pos;
        }

        match c {
            '^' => pos_ptr.y += 1,
            '>' => pos_ptr.x += 1,
            'v' => pos_ptr.y -= 1,
            '<' => pos_ptr.x -= 1,
            _  => continue
        }

        house_set.insert(*pos_ptr);
    }

    println!("Santa visited {} houses", house_set.len());
}

//This is definitely a lesson in multithreading, I should get on it
fn problem_four() {
    let mut input = open_problem_input("../../problems/problem_four.txt");

    let mut gen = Md5::new();

    let mut found = false;
    for n in 0.. {
        let key = fmt::format(format_args!("{}{}", input, n));
        gen.input_str(&key);

        let hash = gen.result_str();

        if !found && hash.starts_with("00000") {
            println!("Lowest integer to produce desired hash is {}", n);
            found = true;
        } else if hash.starts_with("000000") {
            println!("Lowest integer to produce desired hash is {}", n);
            break;
        }

        gen.reset();
    }
}

fn open_problem_input(path: &str) -> String {
    let mut file = File::open(path).unwrap();

    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let len = buffer.len();

    buffer.truncate(len - 1);
    buffer
}
