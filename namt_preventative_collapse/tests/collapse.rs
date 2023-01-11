#[test]
fn early_warning_system_too_small() {
    let vec = vec![2, 1, 2, 7];
    use namt_preventative_collapse::collapse_check;
    let result = collapse_check(vec, 3);
    assert!(!result.is_ok());
}

#[test]
fn early_warning_system_all_values_too_big() {
    let vec = vec![6, 4, 5, 7];
    use namt_preventative_collapse::collapse_check;
    let result = collapse_check(vec, 3);
    assert!(!result.is_ok());
}
