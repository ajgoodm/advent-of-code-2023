/// Convert a char to usize according
/// to the mapping a -> 1, b-> 2, ... A -> 27, ...
pub fn char_to_usize(c: char) -> usize {
    if c.is_uppercase() {
        (c as usize) - 38
    } else {
        (c as usize) - 96
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    fn test_char_to_usize() {
        assert_eq!(char_to_usize('a'), 1);
        assert_eq!(char_to_usize('b'), 2);
        assert_eq!(char_to_usize('A'), 27);
    }
}
