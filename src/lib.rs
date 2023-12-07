/// Adds two numbers together.
/// # Examples
/// ```
/// use arithmetic_yukinari::add;
/// let a = add(1, 2);
/// assert_eq!(a, 3);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
/// Subtracts two numbers.
/// # Examples
/// ```
/// use arithmetic_yukinari::subtract;
/// let s = subtract(1, 2);
/// assert_eq!(s, -1);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
/// Multiplies two numbers.
/// # Examples
/// ```
/// use arithmetic_yukinari::multiply;
/// let m = multiply(2, 3);
/// assert_eq!(m, 6);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_numbers() {
        assert_eq!(add(2, 3), 5);
    }
    #[test]
    fn subtract_numbers() {
        assert_eq!(subtract(2, 3), -1);
    }
    #[test]
    fn multiply_numbers() {
        assert_eq!(multiply(2, 3), 6);
    }
}