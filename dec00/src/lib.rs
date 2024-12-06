pub fn do_something() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_do_something() {
        assert!(do_something());
    }
}
