static INPUT: &'static str = include_str!("../problems/problem_one.txt");

fn main() {
    let (floor, basement_pos) = move_santa(INPUT);

    println!("Santa is at floor #{}", floor);
    println!("Position {} moves Santa into the basement", basement_pos.unwrap());
}

fn move_santa(input: &str) -> (i32, Option<usize>) {
    input.chars().enumerate().fold((0, None), |santa, item| {
        let (instr_pos, instr) = item;

        let movement = match instr {
            '(' => 1,
            ')' => -1,
            _ => 0
        };

        let (curr_floor, basement_instr) = santa;

        let found = if let Some(_) = basement_instr {
            true
        } else {
            false
        };

        let basement_pos = if !found && curr_floor == -1 {
            Some(instr_pos)
        } else {
            basement_instr
        };

        (curr_floor + movement, basement_pos)
    })
}

#[cfg(test)]
mod test {
    use super::INPUT;
    use super::move_santa;

    #[test]
    fn test_move_santa() {
        assert_eq!((74, Some(1795)), move_santa(INPUT));
    }
}
