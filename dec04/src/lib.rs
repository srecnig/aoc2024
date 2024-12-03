pub fn do_something() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_test_something() {
        assert!(true)
    }
}
