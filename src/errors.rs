error_chain! {
    errors {
        InvalidKeyLen(raw_key: String, typename: &'static str) {
            description("Failed to convert string to key because it had the wrong length")
            display("Failed to convert '{}' to '{}' because it had the wrong length", raw_key, typename)
        }
        InvalidKeyChars(raw_key: String, typename: &'static str) {
            description("Failed to convert string to key because it contained invalid characters")
            display("Failed to convert '{}' to '{}' because it contained invalid characters", raw_key, typename)
        }
    }
}
