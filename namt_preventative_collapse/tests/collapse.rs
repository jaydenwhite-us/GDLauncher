#[test]
fn early_warning_system_all_values_too_small() {
    let mut vec = vec![2, 1, 2, 7];
    use namt_preventative_collapse::collapse_check;
    let result = collapse_check(&mut vec, 3);
    assert_ne!(result, Ok(()));
}

#[test]
fn early_warning_system_all_values_too_big() {
    let mut vec = vec![6, 4, 5, 7];
    use namt_preventative_collapse::collapse_check;
    let result = collapse_check(&mut vec, 3);
    assert_ne!(result, Ok(()));
}
#[test]
fn general_warning_system() {
    let mut vec = vec![2, 1, 3, 5, 4, 7, 13];
    use namt_preventative_collapse::collapse_check;
    let result = collapse_check(&mut vec, 3);
    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}

#[test]
fn mini_mine() {
    use std::fs;
    let numbers = fs::read_to_string("../test_inputs/mini_mine.txt");
    let mut numbers = Vec::from_iter(numbers.unwrap().lines().map(|x| x.parse::<u128>().unwrap()));
    use namt_preventative_collapse::collapse_check;
    let result = collapse_check(&mut numbers, 5);

    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}
#[test]
fn mega_mine() {
    use std::fs;
    let numbers = fs::read_to_string("..\\test_inputs\\mega_mine.txt");
    let mut numbers = Vec::from_iter(numbers.unwrap().lines().map(|x| x.parse::<u128>().unwrap()));
    use namt_preventative_collapse::collapse_check;
    let result = collapse_check(&mut numbers, 100);

    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}

#[test]
fn mega_mine_bench_verify_alloc_is_slow() {
    use std::fs;
    let numbers = fs::read_to_string("..\\test_inputs\\mega_mine.txt");
    let mut numbers = Vec::from_iter(numbers.unwrap().lines().map(|x| x.parse::<u128>().unwrap()));
    use namt_preventative_collapse::bench_iterations::collapse_check_alloc_and_sort_on_each_loop;
    let result = collapse_check_alloc_and_sort_on_each_loop(&mut numbers, 100);

    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}

#[test]
fn mega_mine_are_references_better() {
    use std::fs;
    let numbers = fs::read_to_string("..\\test_inputs\\mega_mine.txt");
    let mut numbers = Vec::from_iter(numbers.unwrap().lines().map(|x| x.parse::<u128>().unwrap()));
    use namt_preventative_collapse::bench_iterations::collapse_check_are_references_better;

    let result = collapse_check_are_references_better(&mut numbers, 100);

    if !result.is_ok() {
        eprintln!("{:?}", result);
    }

    assert_ne!(result, Ok(()));
}

#[test]
fn refs_early_warning_system_all_values_too_small() {
    let mut vec = vec![2, 1, 2, 7];
    use namt_preventative_collapse::bench_iterations::collapse_check_are_references_better;
    let result = collapse_check_are_references_better(&mut vec, 3);
    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}

#[test]
fn refs_early_warning_system_all_values_too_big() {
    let mut vec = vec![6, 4, 5, 7];
    use namt_preventative_collapse::bench_iterations::collapse_check_are_references_better;
    let result = collapse_check_are_references_better(&mut vec, 3);
    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}
#[test]
fn refs_general_warning_system() {
    let mut vec = vec![2, 1, 3, 5, 4, 7, 13];
    use namt_preventative_collapse::bench_iterations::collapse_check_are_references_better;
    let result = collapse_check_are_references_better(&mut vec, 3);
    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}

#[test]
fn refs_mini_mine() {
    use std::fs;
    let numbers = fs::read_to_string("../test_inputs/mini_mine.txt");
    let mut numbers = Vec::from_iter(numbers.unwrap().lines().map(|x| x.parse::<u128>().unwrap()));
    use namt_preventative_collapse::bench_iterations::collapse_check_are_references_better;
    let result = collapse_check_are_references_better(&mut numbers, 5);

    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}

#[test]
fn mega_mine_with_more_short_circuiting() {
    use std::fs;
    let numbers = fs::read_to_string("..\\test_inputs\\mega_mine.txt");
    let mut numbers = Vec::from_iter(numbers.unwrap().lines().map(|x| x.parse::<u128>().unwrap()));
    use namt_preventative_collapse::bench_iterations::collapse_check_with_more_short_circuiting;

    let result = collapse_check_with_more_short_circuiting(&mut numbers, 100);

    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}

#[test]
fn mega_mine_narrowing_search() {
    use std::fs;
    let numbers = fs::read_to_string("..\\test_inputs\\mega_mine.txt");
    let mut numbers = Vec::from_iter(numbers.unwrap().lines().map(|x| x.parse::<u128>().unwrap()));
    use namt_preventative_collapse::bench_iterations::collapse_check_narrowing_search;

    let result = collapse_check_narrowing_search(&mut numbers, 100);

    if !result.is_ok() {
        eprintln!("{:?}", result);
    }
    assert_ne!(result, Ok(()));
}
