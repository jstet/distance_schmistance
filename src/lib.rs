use pyo3::prelude::*;

/// Calculates the Wagner-Fischer distance between two strings.
///
/// The Wagner-Fischer distance is a measure of the minimum number of edit operations (insertions,
/// deletions, and substitutions) required to transform one string into another.
///
/// # Arguments
///
/// * `s1` - The first string.
/// * `s2` - The second string.
/// * `insert_costs` - Optional vector of insertion costs for each character. If not provided, the default cost is 1.0.
/// * `delete_costs` - Optional vector of deletion costs for each character. If not provided, the default cost is 1.0.
/// * `substitute_costs` - Optional 2D vector of substitution costs for each pair of characters. If not provided, the default cost is 1.0.
///
/// # Returns
///
/// The Wagner-Fischer distance between the two strings as a floating-point number.
#[pyfunction]
pub fn _wagner_fischer(
    s1: &str,
    s2: &str,
    insert_costs: Option<Vec<f64>>,
    delete_costs: Option<Vec<f64>>,
    substitute_costs: Option<Vec<Vec<f64>>>,
) -> PyResult<f64> {
    let m = s1.chars().count();
    let n = s2.chars().count();
    let mut dp = vec![vec![0.0; n + 1]; m + 1];

    // Initialize the first row of the DP table with deletion costs
    for i in 1..=m {
        dp[i][0] = dp[i - 1][0] + cost_of_deletion(s1.chars().nth(i - 1).unwrap(), delete_costs.clone());
    }

    // Initialize the first column of the DP table with insertion costs
    for j in 1..=n {
        dp[0][j] = dp[0][j - 1] + cost_of_insertion(s2.chars().nth(j - 1).unwrap(), insert_costs.clone());
    }

    // Calculate the minimum edit distance using dynamic programming
    for i in 1..=m {
        for j in 1..=n {
            if s1.chars().nth(i - 1).unwrap() == s2.chars().nth(j - 1).unwrap() {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = f64::min(
                    dp[i - 1][j] + cost_of_deletion(s1.chars().nth(i - 1).unwrap(), delete_costs.clone()),
                    f64::min(
                        dp[i][j - 1] + cost_of_insertion(s2.chars().nth(j - 1).unwrap(), insert_costs.clone()),
                        dp[i - 1][j - 1]
                            + cost_of_substitution(
                                s1.chars().nth(i - 1).unwrap(),
                                s2.chars().nth(j - 1).unwrap(),
                                substitute_costs.clone(),
                            ),
                    ),
                );
            }
        }
    }

    Ok(dp[m][n])
}

fn cost_of_deletion(_c: char, delete_costs: Option<Vec<f64>>) -> f64 {
    let delete_cost = if let Some(delete_costs) = &delete_costs {
        delete_costs[_c as u8 as usize]
    } else {
        1.0
    };

    delete_cost
}

fn cost_of_insertion(_c: char, insert_costs: Option<Vec<f64>>) -> f64 {
    let insert_cost = if let Some(insert_costs) = &insert_costs {
        insert_costs[_c as u8 as usize]
    } else {
        1.0
    };

    insert_cost
}

fn cost_of_substitution(_c1: char, _c2: char, substitute_costs: Option<Vec<Vec<f64>>>) -> f64 {
    let substitute_cost = if let Some(substitute_costs) = &substitute_costs {
        substitute_costs[_c1 as u8 as usize][_c2 as u8 as usize]
    } else {
        1.0
    };
    substitute_cost
}

/// Initializes the _distance_schmistance Python module.
///
/// # Arguments
///
/// * `_py` - The Python interpreter.
/// * `m` - The module to initialize.
///
/// # Examples
///
/// ```
/// let gil = Python::acquire_gil();
/// let py = gil.python();
/// let m = PyModule::new(py, "_distance_schmistance").unwrap();
/// _distance_schmistance(py, &m).unwrap();
/// ```
#[pymodule]
fn _distance_schmistance(_py: Python, m: &PyModule) -> PyResult<()> {
    // Add the _levenshtein function to the module
    m.add_function(wrap_pyfunction!(_wagner_fischer, m)?)?;
    Ok(())
}