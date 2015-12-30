mod advent_helper;
use advent_helper::open_problem_input;

fn main() {
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
