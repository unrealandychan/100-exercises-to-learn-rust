
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_eq() {
        assert!(42u32.eq(&42u32));
        assert!(!42u32.eq(&43u32));
    }
}
