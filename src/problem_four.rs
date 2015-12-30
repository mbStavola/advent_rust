extern crate crypto;
use crypto::md5::Md5;
use crypto::digest::Digest;

//This is definitely a lesson in multithreading, I should get on it
fn problem_four() {
    let input = open_problem_input("../../problems/problem_four.txt");

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
