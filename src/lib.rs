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
    insert_costs: Vec<f64>,
    delete_costs: Vec<f64>,
    substitute_costs: Vec<Vec<f64>>,
) -> PyResult<f64> {
    let m = s1.chars().count();
    let n = s2.chars().count();
    
    // Use the shorter string as the basis for the rows
    let (shorter, longer) = if m < n {
        (s1, s2)
    } else {
        (s2, s1)
    };
    
    let mut dp = vec![0.0; shorter.len() + 1];
    
    // Initialize the first row of the DP table with deletion costs
    for i in 1..=shorter.len() {
        dp[i] = dp[i - 1] + delete_costs[shorter.chars().nth(i - 1).unwrap() as u8 as usize];
    }
    
    for j in 1..=longer.len() {
        let mut prev = dp[0];
        dp[0] += insert_costs[longer.chars().nth(j - 1).unwrap() as u8 as usize];
        
        for i in 1..=shorter.len() {
            let temp = dp[i];
            
            if shorter.chars().nth(i - 1).unwrap() == longer.chars().nth(j - 1).unwrap() {
                dp[i] = prev;
            } else {
                dp[i] = f64::min(
                    prev + delete_costs[shorter.chars().nth(i - 1).unwrap() as u8 as usize],
                    f64::min(
                        dp[i] + insert_costs[longer.chars().nth(j - 1).unwrap() as u8 as usize],
                        dp[i - 1] + substitute_costs[shorter.chars().nth(i - 1).unwrap() as u8 as usize][
                            longer.chars().nth(j - 1).unwrap() as u8 as usize
                        ]
                        ),
                    )
            }
            
            prev = temp;
        }
    }
    
    Ok(dp[shorter.len()])
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