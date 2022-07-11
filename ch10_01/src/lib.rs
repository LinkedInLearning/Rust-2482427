/// Première ligne de documentation de la
/// fonction add
///
/// ```
/// let result = ch10_01::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let reponse = add(5, 7);
        assert_eq!(reponse, 12);
    }
}
