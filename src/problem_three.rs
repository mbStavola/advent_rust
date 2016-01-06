use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Pair {
    x: i32,
    y: i32
}

fn main() {
    let input = include_str!("../problems/problem_three.txt");

    let mut santa_curr_pos = Pair { x: 0, y: 0 };
    let mut robo_curr_pos = Pair { x: 0, y: 0 };

    let mut house_set: HashSet<Pair> = HashSet::new();
    house_set.insert(santa_curr_pos);

    let mut is_santa = true;
    for c in input.chars() {
        let mut pos_ptr: &mut Pair;
        if is_santa {
            pos_ptr = &mut santa_curr_pos;
        } else {
            pos_ptr = &mut robo_curr_pos;
        }

        match c {
            '^' => pos_ptr.y += 1,
            '>' => pos_ptr.x += 1,
            'v' => pos_ptr.y -= 1,
            '<' => pos_ptr.x -= 1,
            _  => continue
        }

        is_santa = !is_santa;
        house_set.insert(*pos_ptr);
    }

    println!("Santa visited {} houses", house_set.len());
}
