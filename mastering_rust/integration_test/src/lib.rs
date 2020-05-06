//! This crate provides functionality for adding things
//!
//! # Examples
//! ```
//! use integration_test::sum;
//!
//! let work_a = 4;
//! let work_b = 34;
//! let total_work = sum(work_a, work_b);
//! ```

/// Sum two arguments
///
/// # Examples
/// ```
/// assert_eq!(integration_test::sum(1, 1), 2);
/// ```
pub fn sum(a: i8, b: i8) -> i8 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
