#![feature(test)]

extern crate test;
use test::Bencher;
fn setup() -> (Vec<u128>, usize) {
    use std::fs;
    let numbers = fs::read_to_string("../test_inputs/mega_mine.txt");
    let numbers = Vec::from_iter(numbers.unwrap().lines().map(|x| x.parse::<u128>().unwrap()));
    (numbers, 100)
}
#[bench]
fn namt_current(bencher: &mut Bencher) {
    let (mut numbers, period) = setup();
    use namt_preventative_collapse::collapse_check;
    bencher.iter(|| collapse_check(&mut numbers, period));
}
#[bench]
#[ignore]
fn namt_verify_many_allocations_is_slow(bencher: &mut Bencher) {
    let (mut numbers, period) = setup();
    use namt_preventative_collapse::bench_iterations::collapse_check_alloc_and_sort_on_each_loop;
    bencher.iter(|| collapse_check_alloc_and_sort_on_each_loop(&mut numbers, period));
}

#[bench]
//Not really. It's slower on average, with a wider standard deviation. But the developer experience was nicer.
fn namt_are_references_better(bencher: &mut Bencher) {
    let (mut numbers, period) = setup();
    use namt_preventative_collapse::bench_iterations::collapse_check_are_references_better;
    bencher.iter(|| collapse_check_are_references_better(&mut numbers, period));
}

//Nope
#[bench]
fn more_short_circuit(bencher: &mut Bencher) {
    let (mut numbers, period) = setup();
    use namt_preventative_collapse::bench_iterations::collapse_check_with_more_short_circuiting;
    bencher.iter(|| collapse_check_with_more_short_circuiting(&mut numbers, period));
}
//uh-uh
#[bench]
fn namt_local_alloc_with_no_short_circuit(bencher: &mut Bencher) {
    let (mut numbers, period) = setup();
    use namt_preventative_collapse::bench_iterations::collapse_check_no_short_ciruit;
    bencher.iter(|| collapse_check_no_short_ciruit(&mut numbers, period));
}
//uh-uh
#[bench]
fn namt_narrowing_binary_search(bencher: &mut Bencher) {
    let (mut numbers, period) = setup();
    use namt_preventative_collapse::bench_iterations::collapse_check_narrowing_search;
    bencher.iter(|| collapse_check_narrowing_search(&mut numbers, period));
}
