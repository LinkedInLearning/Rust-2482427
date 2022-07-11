/// Première ligne de documentation de la
/// fonction add
///
/// ```
/// let result = ch10_02::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    println!("ceci est un print");
    a + b
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let reponse = add(5, 7);
        assert_eq!(reponse, 12);
    }

    #[test]
    #[ignore]
    fn test_sub() {
        let reponse = sub(5, 7);
        assert_eq!(reponse, -2);
    }
}
