use regex::Regex;

pub fn detect_multiplications(haystack: &str) -> Vec<&str> {
    // let mut mul
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let captures: Vec<&str> = re.find_iter(haystack).map(|mat| mat.as_str()).collect();
    captures
}

pub fn multiply(instruction: &str) -> i32 {
    let re = Regex::new(r"mul\((?P<left>\d+),(?P<right>\d+)\)").unwrap();
    let numbers = re.captures(instruction).unwrap();
    let left: i32 = numbers["left"].parse().unwrap();
    let right: i32 = numbers["right"].parse().unwrap();
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_detect_multiplications() {
        assert_eq!(
            vec!["mul(2,4)", "mul(3,12)"],
            detect_multiplications(&String::from("xmul(2,4)%&mul[3,7]!mul(3,12)"))
        )
    }

    #[test]
    fn can_multiply() {
        assert_eq!(8, multiply(&String::from("mul(2,4)")));
        assert_eq!(0, multiply(&String::from("mul(2,0)")));
        assert_eq!(25000, multiply(&String::from("mul(50,500)")));
    }
}
