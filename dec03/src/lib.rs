use regex::Regex;

pub fn apply_conditionals(haystack: &str) -> String {
    let mut proper_haystack = haystack.to_string();

    let re = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let captures: Vec<regex::Match> = re.find_iter(haystack).collect();
    for m in captures.into_iter().rev() {
        proper_haystack.replace_range(m.start()..m.end(), "");
    }

    proper_haystack
}

pub fn detect_multiplications(haystack: &str) -> Vec<String> {
    // let mut mul
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let captures: Vec<String> = re
        .find_iter(haystack)
        .map(|mat| mat.as_str().to_string())
        .collect();
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
    fn can_apply_conditionals() {
        assert_eq!(
            "xmul(2,4)&mul[3,7]!^?mul(8,5))",
            apply_conditionals(&String::from(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
            ))
        );
        assert_eq!(
            String::from("blablablab_1do()2do()3_4567_pepi"),
            apply_conditionals(&String::from(
                "blablablab_don't()blablablado()1do()2do()3_4567_don't()hugohugoghuo_do()pepi",
            ))
        );
    }

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
