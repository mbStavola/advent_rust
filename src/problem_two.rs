mod advent_helper;
use advent_helper::open_problem_input;

fn main() {
    let input = open_problem_input("../../problems/problem_two.txt");

    let mut paper_length = 0;
    let mut ribbon_length = 0;

    for line in input.split("\n") {
        let dimens: Vec<u32> = line.split("x").map(|e| e.parse::<u32>().unwrap()).collect();

        paper_length += length_of_paper(&dimens);
        ribbon_length += length_of_ribbon(&dimens);
    }

    println!("{} sq ft of wrapping paper needed", paper_length);
    println!("{} ft of ribbon needed", ribbon_length);
}

fn length_of_paper(dimens: &Vec<u32>) -> u32 {
    //2*l*w + 2*w*h + 2*h*l
    let side_one = dimens[0] * dimens[1];
    let side_two = dimens[1] * dimens[2];
    let side_three = dimens[0] * dimens[2];

    let mut slack = if side_one > side_two { side_one } else { side_two };
    if slack < side_three {
        slack = side_three;
    }

    2 * (side_one + side_two + side_three) + slack
}

fn length_of_ribbon(dimens: &Vec<u32>) -> u32 {
    let dimen_max: &u32 = dimens.iter().max().unwrap();
    let ribbon_wrap = 2 * (dimens[0] + dimens[1] + dimens[2] - dimen_max);

    (dimens[0] * dimens[1] * dimens[2]) + ribbon_wrap
}
