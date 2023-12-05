use pyo3::prelude::*;


/// Based on https://github.com/febeling/edit-distance
#[pyfunction]
fn _levenshtein(a: &str, b: &str) -> PyResult<usize> {
    let len_a = a.chars().count();
    let len_b = b.chars().count();
    if len_a < len_b {
        return _levenshtein(b, a);
    }
    // handle special case of 0 length
    if len_a == 0 {
        return Ok(len_b);
    } else if len_b == 0 {
        return Ok(len_a);
    }

    let len_b = len_b + 1;

    let mut pre;
    let mut tmp;
    let mut cur = vec![0; len_b];

    // initialize string b
    for i in 1..len_b {
        cur[i] = i;
    }

    // calculate edit distance
    for (i, ca) in a.chars().enumerate() {
        // get first column for this row
        pre = cur[0];
        cur[0] = i + 1;
        for (j, cb) in b.chars().enumerate() {
            tmp = cur[j + 1];
            cur[j + 1] = std::cmp::min(
                // deletion
                tmp + 1,
                std::cmp::min(
                    // insertion
                    cur[j] + 1,
                    // match or substitution
                    pre + if ca == cb { 0 } else { 1 },
                ),
            );
            pre = tmp;
        }
    }
    Ok(cur[len_b - 1])
}

/// A Python module implemented in Rust.
#[pymodule]
fn _distance_schmistance(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(_levenshtein, m)?)?;
    Ok(())
}
