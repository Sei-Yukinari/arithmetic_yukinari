/// Adds two numbers together.
/// # Examples
/// ```
/// let a = add(1, 2);
/// assert_eq!(a, 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}