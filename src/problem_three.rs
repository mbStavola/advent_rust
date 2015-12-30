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
