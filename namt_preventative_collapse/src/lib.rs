pub fn collapse_check(vector: Vec<usize>, period: usize) -> Result<(), &'static str> {
    let mut values: Vec<(&usize, usize)> = vector[0..period].iter().zip(0..period).collect(); //O(n)
    values.sort_by(|(entry_a, _), (entry_b, _)| entry_a.cmp(entry_b)); //O(n*log(n))

    for c in period - 1..vector.len() {
        '_ensure_sorting: for x in 0..values.len() - 1 {
            //avoiding nightly build of Rust for is_sorted();
            assert!(values[x].0 <= values[x + 1].0);
            //The logic after this assertion block relies on the assumption the values are sorted.
        } // O(n)

        let max = values[0].0;
        let min = values[period - 1].0;

        //Early warning
        match vector[c] / 2 {
            a if a < *max => {
                return Err("Imminent Failure detected. Maximum too small to reach required sum.")
            }

            a if *min > a => {
                return Err("Imminent Failure detected Minimum to big to reach required sum.")
            }
            a => {}
        }
    }
    Ok(())
}
