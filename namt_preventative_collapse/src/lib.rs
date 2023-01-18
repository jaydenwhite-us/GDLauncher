//!
//! # Summary
//! namt_preventative_collapse is an early warning system for mining operations.
//! Best implemented in all hardware that comes into contact with rocks, or receives mining data.
//! For most applications `collapse_check` will suffice, but the underlying search function `pairs_exist` is
//! exported for convenience.
use std::vec;
/// Checks for imminent mine collapse of mining operation.
/// ### Arguments
/// * `vector:` A vector
/// * `period:` Determines the range `i..i+k`, where i+k is not included.
/// ### Example
/// ```
/// use namt_preventative_collapse::collapse_check;
/// let mut vector = vec![1,2,3,4];
/// let result = collapse_check(&mut vector, 3);
/// assert_eq!(true, result.is_ok());
///
/// let mut vector = vec![1,1,1,4];
/// let period = 3;
/// let result = collapse_check(&mut vector, 3);
/// assert_eq!(false, result.is_ok());
/// ```
pub fn collapse_check(vector: &mut Vec<u128>, period: usize) -> Result<(), String> {
    if vector.len() <= period {
        return Ok(()); //Safe.
    }
    let mut values: Vec<(u128, usize)> = vec![]; //O(n)
    for x in 0..period {
        values.push((vector[x], x.try_into().unwrap()));
    }

    for c in period..vector.len() {
        values.sort_by(|(entry_a, _), (entry_b, _)| entry_a.cmp(entry_b)); //O(n*log(n)) first time. O(n) every time after.

        '_ensure_sorting: for x in 0..period - 1 {
            assert!(values[x].0 <= values[x + 1].0);
            // Scaffolding. Do not remove. Any logic after this assertion block relies on the assumption the values are sorted.
            // Also avoiding nightly build of Rust, .is_sorted();
        } // O(n)

        let target = vector[c];
        let result = pair_exists(&values, target);
        if result.is_err() {
            eprintln!("Index: {}, Element: {} ", c, vector[c]);
            return result;
        }

        //Decrement indexes, and insert (vector[c], period -1) in the place of (some_element, 0).
        for x in 0..values.len() {
            if values[x].1 != 0 {
                values[x].1 -= 1;
                continue;
            }
            values[x] = (target.clone(), period - 1);
        } //O(n)
          //remove zero index
        for x in 0..values.len() {
            assert!(values[x].1 < period, "Logic Error: Indexing operations.");
        }
    }
    Ok(())
}

///     
/// Checks for existence of `a` and `b` in `values` such that `a + b = target`, and returns a Result.
/// Uses binary_search which assumes `values` is sorted by the predicate `values[n] <= values[n+1]`, otherwise the result is meaningless.
///
pub fn pair_exists(values: &Vec<(u128, usize)>, target: u128) -> Result<(), String> {
    let min = values[0].0;
    let max = values[values.len() - 1].0;
    match target {
        //Early warnings
        //Match is used for maintainance clarity of cases.
        target if 2 * max < target => {
            Err("Imminent Failure: Maximum is too small to reach the required sum.".to_string())
        }
        target if 2 * min > target => {
            Err("Imminent Failure: Minimum is too big to reach the required sum.".to_string())
        }
        _ => {
            //Normal warning. Is c-a in the values?
            for (element, _) in values.iter() {
                let inverse = match target >= *element {
                    true => target - element,
                    false => element - target,
                }; //Technically an absolute value would work here.

                if values
                    .binary_search_by(|tuple_ptr| tuple_ptr.0.cmp(&inverse))
                    .is_ok()
                {
                    return Ok(());
                }
            } //O(n*log(n)) -> On average, a non-faulty mine will hit this case.
            Err("Imminent Failure detected. No pairs reach the required sum.".to_string())
        }
    }
}

///A module specifically for benching multiple iterations of the collapse_check function.
pub mod bench_iterations {
    use std::vec;
    ///Allocates on each iteration of the loop. Verifying a bad idea.
    pub fn collapse_check_alloc_and_sort_on_each_loop(
        vector: &mut Vec<u128>,
        period: usize,
    ) -> Result<(), String> {
        for c in period..vector.len() {
            let mut values: Vec<(u128, usize)> = vec![]; //O(n)
            for x in 0..c {
                values.push((vector[x], x.try_into().unwrap())); //have to fix this clone.
            }
            values.sort_by(|(entry_a, _), (entry_b, _)| entry_a.cmp(entry_b)); //O(n*log(n)) first time. O(n) every time after.

            '_ensure_sorting: for x in 0..period - 1 {
                assert!(values[x].0 <= values[x + 1].0);
                // Scaffolding. Do not remove. Any logic after this assertion block relies on the assumption the values are sorted.
                // Also avoiding nightly build of Rust, .is_sorted();
            } // O(n)

            let target = vector[c];
            let result = super::pair_exists(&values, target);
            if result.is_err() {
                return result;
            }
        }
        Ok(())
    }

    //No. It's slower on average.
    ///Does not allocate a local vector. Uses only references.
    pub fn collapse_check_are_references_better(
        vector: &mut Vec<u128>,
        period: usize,
    ) -> Result<(), String> {
        if vector.len() <= period {
            return Ok(()); //Safe.
        }
        let (safe, r#unsafe) = vector.split_at_mut(period);
        let mut safe: Vec<(&u128, usize)> = safe.iter().zip(0..period).collect(); //O(n)

        for element in r#unsafe.iter() {
            safe.sort_by(|(entry_a, _), (entry_b, _)| (entry_a).cmp(entry_b)); //O(n*log(n)) first time. O(n) every time after.

            '_ensure_sorting: for x in 0..period - 1 {
                assert!(safe[x].0 <= safe[x + 1].0);
                // Scaffolding. Do not remove. Any logic after this assertion block relies on the assumption the values are sorted.
                // Also avoiding nightly build of Rust, .is_sorted();
            } // O(n)

            let result = pair_exists_references(&safe, *element);
            if result.is_err() {
                return result;
            }

            //Decrement indexes, and insert (vector[c], period -1) in the place of (some_element, 0).
            for x in 0..safe.len() {
                if safe[x].1 != 0 {
                    safe[x].1 -= 1;
                    continue;
                }
                safe[x] = (element, period - 1);
            } //O(n)
              //remove zero index
            for x in 0..safe.len() {
                assert!(safe[x].1 < period, "Logic Error: Indexing operations.");
            }
        }
        Ok(())
    }

    //No
    ///Includes more short circuits to see if improving on the best case scenario is worthwhile.
    pub fn collapse_check_with_more_short_circuiting(
        vector: &mut Vec<u128>,
        period: usize,
    ) -> Result<(), String> {
        if vector.len() <= period {
            return Ok(()); //Safe.
        }
        let (safe, r#unsafe) = vector.split_at_mut(period);
        let mut safe: Vec<(&u128, usize)> = safe.iter().zip(0..period).collect(); //O(n)

        for element in r#unsafe.iter() {
            safe.sort_by(|(entry_a, _), (entry_b, _)| (entry_a).cmp(entry_b)); //O(n*log(n)) first time. O(n) every time after.

            '_ensure_sorting: for x in 0..period - 1 {
                assert!(safe[x].0 <= safe[x + 1].0);
                // Scaffolding. Do not remove. Any logic after this assertion block relies on the assumption the values are sorted.
                // Also avoiding nightly build of Rust, .is_sorted();
            } // O(n)

            let result = pair_exists_references_with_short_circuit(&safe, *element);
            if result.is_err() {
                return result;
            }

            //Decrement indexes, and insert (vector[c], period -1) in the place of (some_element, 0).
            for x in 0..safe.len() {
                if safe[x].1 != 0 {
                    safe[x].1 -= 1;
                    continue;
                }
                safe[x] = (element, period - 1);
            } //O(n)
              //remove zero index
            for x in 0..safe.len() {
                assert!(safe[x].1 < period, "Logic Error: Indexing operations.");
            }
        }
        Ok(())
    }

    //No, 'cause short circuiting is a best case scenario. Is basically about the same speed.
    ///Removes all short circuiting.
    pub fn collapse_check_no_short_ciruit(
        vector: &mut Vec<u128>,
        period: usize,
    ) -> Result<(), String> {
        if vector.len() <= period {
            return Ok(()); //Safe.
        }
        let mut values: Vec<(u128, usize)> = vec![]; //O(n)
        for x in 0..period {
            values.push((vector[x], x.try_into().unwrap())); //have to fix this clone.
        }

        for c in period..vector.len() {
            values.sort_by(|(entry_a, _), (entry_b, _)| entry_a.cmp(entry_b)); //O(n*log(n)) first time. O(n) every time after.

            '_ensure_sorting: for x in 0..period - 1 {
                assert!(values[x].0 <= values[x + 1].0);
                // Scaffolding. Do not remove. Any logic after this assertion block relies on the assumption the values are sorted.
                // Also avoiding nightly build of Rust, .is_sorted();
            } // O(n)

            let target = vector[c];
            let result = pair_exists_no_short_circuit(&values, target);
            if result.is_err() {
                return result;
            }

            //Decrement indexes, and insert (vector[c], period -1) in the place of (some_element, 0).
            for x in 0..values.len() {
                if values[x].1 != 0 {
                    values[x].1 -= 1;
                    continue;
                }
                values[x] = (target.clone(), period - 1);
            } //O(n)
              //remove zero index
            for x in 0..values.len() {
                assert!(values[x].1 < period, "Logic Error: Indexing operations.");
            }
        }
        Ok(())
    }

    //Also, no. Worth revisiting as it may reduce complexity as n grows.
    ///Takes advantage of the sorted list to narrow the search area of the binary search as it goes.
    /// Should improve on the worst case scenario.
    pub fn collapse_check_narrowing_search(
        vector: &mut Vec<u128>,
        period: usize,
    ) -> Result<(), String> {
        if vector.len() <= period {
            return Ok(()); //Safe.
        }
        let mut values: Vec<(u128, usize)> = vec![]; //O(n)
        for x in 0..period {
            values.push((vector[x], x.try_into().unwrap())); //have to fix this clone.
        }

        for c in period..vector.len() {
            values.sort_by(|(entry_a, _), (entry_b, _)| entry_a.cmp(entry_b)); //O(n*log(n)) first time. O(n) every time after.

            '_ensure_sorting: for x in 0..period - 1 {
                assert!(values[x].0 <= values[x + 1].0);
                // Scaffolding. Do not remove. Any logic after this assertion block relies on the assumption the values are sorted.
                // Also avoiding nightly build of Rust, .is_sorted();
            } // O(n)

            let target = vector[c];
            let result = pair_exists_narrowing(&values, target);
            if result.is_err() {
                return result;
            }

            //Decrement indexes, and insert (vector[c], period -1) in the place of (some_element, 0).
            for x in 0..values.len() {
                if values[x].1 != 0 {
                    values[x].1 -= 1;
                    continue;
                }
                values[x] = (target.clone(), period - 1);
            } //O(n)
              //remove zero index
            for x in 0..values.len() {
                assert!(values[x].1 < period, "Logic Error: Indexing operations.");
            }
        }
        Ok(())
    }

    ///Used in collapse_check_are_references_better.
    fn pair_exists_references(values: &Vec<(&u128, usize)>, target: u128) -> Result<(), String> {
        let min = values[0].0;
        let max = values[values.len() - 1].0;
        match target {
            //Early warnings
            //Match solely for maintainance clarity, there are really only three cases.
            target if 2 * max < target => {
                Err("Imminent Failure: Maximum is too small to reach the required sum.".to_string())
            }
            target if 2 * min > target => {
                Err("Imminent Failure: Minimum is too big to reach the required sum.".to_string())
            }
            _ => {
                //Normal warning. Is c-a in the values?((
                for (element, _) in values.iter() {
                    let inverse = match &target >= *element {
                        true => target - **element,
                        false => **element - target,
                    };

                    if values
                        .binary_search_by(|tuple_ptr| tuple_ptr.0.cmp(&inverse))
                        .is_ok()
                    {
                        return Ok(());
                    }
                } //O(n*log(n)) -> On average, a non-faulty mine will hit this case.
                Err("Imminent Failure detected. No pairs reach the required sum.".to_string())
            }
        }
    }

    ///Used in collapse_check_with_more_short_circuiting
    fn pair_exists_references_with_short_circuit(
        values: &Vec<(&u128, usize)>,
        target: u128,
    ) -> Result<(), String> {
        let mut i = 0;
        let min = values[i].0;
        let max = values[values.len() - 1].0;
        //Normal warning. Is c-a in the values?((
        for (element, _) in values.iter() {
            if 2 * max < target {
                return Err(
                    "Imminent Failure: Maximum is too small to reach the required sum.".to_string(),
                );
            }
            if 2 * min > target {
                return Err(
                    "Imminent Failure: Minimum is too big to reach the required sum.".to_string(),
                );
            }
            let inverse = match &target >= *element {
                true => target - **element,
                false => **element - target,
            };

            if values
                .binary_search_by(|tuple_ptr| tuple_ptr.0.cmp(&inverse))
                .is_ok()
            {
                return Ok(());
            }
            i += 1;
        } //O(n*log(n)) -> On average, a non-faulty mine will hit this case.
        Err("Imminent Failure detected. No pairs reach the required sum.".to_string())
    }

    ///Used in collapse_check_no_short_ciruit
    fn pair_exists_no_short_circuit(
        values: &Vec<(u128, usize)>,
        target: u128,
    ) -> Result<(), String> {
        //Normal warning. Is c-a in the values?
        for (element, _) in values.iter() {
            let inverse = match target >= *element {
                true => target - element,
                false => element - target,
            };

            if values
                .binary_search_by(|tuple_ptr| tuple_ptr.0.cmp(&inverse))
                .is_ok()
            {
                return Ok(());
            }
        } //O(n*log(n)) -> On average, a non-faulty mine will hit this case.
        Err("Imminent Failure detected. No pairs reach the required sum.".to_string())
    }

    ///Used in collapse_check_narrowing_search
    fn pair_exists_narrowing(values: &Vec<(u128, usize)>, target: u128) -> Result<(), String> {
        let mut i = 0;
        let max = values[values.len() - 1].0;
        //Normal warning. Is c-a in the values?
        for (element, _) in values.iter() {
            if 2 * max < target {
                return Err(
                    "Imminent Failure: Maximum is too small to reach the required sum.".to_string(),
                );
            }
            if 2 * values[i].0 > target {
                return Err(
                    "Imminent Failure: Minimum is too big to reach the required sum.".to_string(),
                );
            }
            let inverse = match target >= *element {
                true => target - element,
                false => element - target,
            };

            if values[i..]
                .binary_search_by(|tuple_ptr| tuple_ptr.0.cmp(&inverse))
                .is_ok()
            {
                return Ok(());
            }
            i += 1
        } //O(n*log(n)) -> On average, a non-faulty mine will hit this case.
        Err("Imminent Failure detected. No pairs reach the required sum.".to_string())
    }
}
