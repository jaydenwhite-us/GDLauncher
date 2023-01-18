fn main() {
    use std::fs;
    let numbers = fs::read_to_string("../../test_inputs/challenge_input.txt");
    let mut numbers = Vec::from_iter(numbers.unwrap().lines().map(|x| x.parse::<u128>().unwrap()));

    use namt_preventative_collapse::collapse_check;
    match collapse_check(&mut numbers, 100) {
        Err(error) => eprintln!("{:?}", error),
        _ => {}
    }
}
