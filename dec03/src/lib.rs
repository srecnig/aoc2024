use regex::Regex;

pub fn apply_conditionals(haystack: &str) -> String {
    let proper_haystack = format!("{}{}{}", "do()", haystack, "don't()");
    let re = Regex::new(r"do\(\).*?don't\(\)").unwrap();
    let captures: Vec<&str> = re
        .find_iter(&proper_haystack)
        .map(|mat| mat.as_str())
        .collect();
    captures.join("")
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
            "do()do()1don't()do()3don't()",
            apply_conditionals(&String::from("do()1don't()2do()3don't()4"))
        );
        assert_eq!(
            "do()1don't()do()3don't()",
            apply_conditionals(&String::from("1don't()2do()3"))
        );
        assert_eq!(
            "do()1don't()do()3do()4don't()",
            apply_conditionals(&String::from("1don't()2do()3do()4"))
        );
        assert_eq!(
            "do()1don't()",
            apply_conditionals(&String::from("1don't()2don't()3don't()4"))
        );
        let demo = String::from(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(
            "do()xmul(2,4)&mul[3,7]!^don't()do()?mul(8,5))don't()",
            apply_conditionals(&demo)
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
