use uuid::Uuid;

// Defaults to post-fix
pub fn generate_id(identifier: &str, insertion: bool) -> String {
    let id = Uuid::new_v4();

    if insertion == true {
        format!("{}-{}", identifier, id)
    } else {
        format!("{}-{}", id, identifier)
    }
} 

#[cfg(test)]
mod tests {
    use crate::generate_id;
    /// Test that  generate_user_id starts with the identifier
    /// when prefix is false
    #[test]
    fn generate_user_id_prefix() {
        let result = generate_id("clear", true).starts_with("clear");

        assert!(result)
    }

    /// Test that  generate_user_id ends with the identifier
    /// when prefix is false
    #[test]
      fn generate_user_id_no_prefix() {
        let result = generate_id("clear", false).ends_with("clear");

        assert!(result)
    }
}
