/// Returns the length of the string passed as an immutable reference.
/// 
/// This exercise demonstrates how to take a reference to a `String` 
/// without taking ownership of it, allowing the original owner to keep using it.
pub fn get_length(s: &String) -> usize {
    s.len()
}

/// Appends the string " world" (with a leading space) to the provided mutable string reference.
/// 
/// This exercise demonstrates how to take a mutable reference to a `String`
/// in order to modify its contents in place.
pub fn append_world(s: &mut String) {
    s.push_str(" world");
}

/// Clones the input string, appends "! (modified)" to the clone, and returns the clone.
/// 
/// This exercise demonstrates how to create a deep copy (clone) of a `String`
/// when you need to modify it but only have an immutable reference to the original.
pub fn clone_and_modify(s: &String) -> String {
    let mut clone = s.clone();
    clone.push_str("! (modified)");
    clone
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_length() {
        let s = String::from("hello");
        assert_eq!(get_length(&s), 5);
        // ownership not moved, s is still available here
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_append_world() {
        let mut s = String::from("hello");
        append_world(&mut s);
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_clone_and_modify() {
        let original = String::from("Rust");
        let modified = clone_and_modify(&original);
        assert_eq!(modified, "Rust! (modified)");
        // original should remain unchanged
        assert_eq!(original, "Rust");
    }
}
