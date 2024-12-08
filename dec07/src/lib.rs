use radix_fmt::*;

#[derive(Debug)]
pub struct Equation {
    pub result: i64,
    pub numbers: Vec<i64>,
}

impl Equation {
    pub fn check(&self, concatenation: bool) -> bool {
        let operator_count = self.numbers.len() - 1;
        let operator_set = OperatorSet::new(2, operator_count as u32);

        for operators in operator_set {
            if self.verify_operators(&operators) {
                return true;
            }
        }

        if concatenation {
            let operator_set = OperatorSet::new(3, operator_count as u32);

            for operators in operator_set {
                if self.verify_operators(&operators) {
                    return true;
                }
            }
        }

        false
    }

    fn verify_operators(&self, operators: &Vec<Operator>) -> bool {
        let mut result: i64 = self.numbers[0];

        for (i, operator) in operators.iter().enumerate() {
            result = match operator {
                Operator::Addition => result + self.numbers[i + 1],
                Operator::Multiplication => result * self.numbers[i + 1],
                Operator::Concatenation => {
                    let new_result = format!("{}{}", result, self.numbers[i + 1]);
                    new_result.parse().unwrap()
                }
            };
            // as soon as we're higher than the result, we know it's false
            if result > self.result {
                return false;
            }
        }
        result == self.result
    }
}

#[derive(Debug, PartialEq)]

enum Operator {
    Addition,
    Multiplication,
    Concatenation,
}

struct OperatorSet {
    base: u32,
    count: u32,
    curr: i32,
}

impl OperatorSet {
    fn new(base: u32, count: u32) -> OperatorSet {
        let curr: i32 = (base.pow(count) - 1) as i32;
        OperatorSet { base, curr, count }
    }
}

impl Iterator for OperatorSet {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < 0 {
            return None;
        }

        let control_string = match self.base {
            2 => format!("{:0width$b}", self.curr, width = self.count as usize),
            3 => {
                let non_padded = format!("{}", radix_3(self.curr));
                let leading_zeros = self.count - non_padded.len() as u32;
                format!("{}{}", "0".repeat(leading_zeros as usize), non_padded)
            }
            _ => {
                panic!()
            }
        };

        let operator_set: Vec<Operator> = control_string
            .chars()
            .map(|ch| match ch {
                '2' => Operator::Concatenation,
                '1' => Operator::Multiplication,
                '0' => Operator::Addition,
                _ => {
                    panic!()
                }
            })
            .collect();

        self.curr -= 1;
        Some(operator_set)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_check_equation() {
        let bad_equation = Equation {
            result: 161011,
            numbers: vec![16, 10, 13],
        };
        assert!(!bad_equation.check(false));
        let good_equation = Equation {
            result: 292,
            numbers: vec![11, 6, 16, 20],
        };
        assert!(good_equation.check(false));
    }
    #[test]
    fn can_check_equation_with_concatenation() {
        let equation = Equation {
            result: 156,
            numbers: vec![15, 6],
        };
        assert!(!equation.check(false));
        assert!(equation.check(true));

        let equation = Equation {
            result: 7290,
            numbers: vec![6, 8, 6, 15],
        };
        assert!(!equation.check(false));
        assert!(equation.check(true));

        let equation = Equation {
            result: 192,
            numbers: vec![17, 8, 14],
        };
        assert!(!equation.check(false));
        assert!(equation.check(true));
    }

    #[test]
    fn can_verify_operators() {
        let equation = Equation {
            result: 190,
            numbers: vec![10, 19],
        };
        assert!(!equation.verify_operators(&vec![Operator::Addition]));
        assert!(equation.verify_operators(&vec![Operator::Multiplication]));

        let equation = Equation {
            result: 3267,
            numbers: vec![81, 40, 27],
        };
        assert!(equation.verify_operators(&vec![Operator::Addition, Operator::Multiplication]));
        assert!(equation.verify_operators(&vec![Operator::Multiplication, Operator::Addition]));
        assert!(!equation.verify_operators(&vec![Operator::Addition, Operator::Addition]));
    }

    #[test]
    fn can_verify_operators_with_concatenation() {
        let equation = Equation {
            result: 156,
            numbers: vec![15, 6],
        };
        assert!(equation.verify_operators(&vec![Operator::Concatenation]));

        let equation = Equation {
            result: 7290,
            numbers: vec![6, 8, 6, 15],
        };
        assert!(equation.verify_operators(&vec![
            Operator::Multiplication,
            Operator::Concatenation,
            Operator::Multiplication
        ]));

        let equation = Equation {
            result: 192,
            numbers: vec![17, 8, 14],
        };
        assert!(equation.verify_operators(&vec![Operator::Concatenation, Operator::Addition,]));
    }

    #[test]
    fn can_iterate_operator_set() {
        let operator_set: Vec<Vec<Operator>> = OperatorSet::new(2, 4).collect();
        assert_eq!(16, operator_set.len());
        assert_eq!(
            vec![
                Operator::Multiplication,
                Operator::Multiplication,
                Operator::Multiplication,
                Operator::Multiplication
            ],
            operator_set[0]
        );
        assert_eq!(
            vec![
                Operator::Addition,
                Operator::Addition,
                Operator::Addition,
                Operator::Addition,
            ],
            operator_set[15]
        );
    }

    #[test]
    fn can_iterate_operator_set_base3() {
        let operator_set: Vec<Vec<Operator>> = OperatorSet::new(3, 3).collect();
        assert_eq!(27, operator_set.len());
        assert_eq!(
            vec![
                Operator::Concatenation,
                Operator::Concatenation,
                Operator::Concatenation,
            ],
            operator_set[0]
        );
        assert_eq!(
            vec![
                Operator::Concatenation,
                Operator::Concatenation,
                Operator::Multiplication,
            ],
            operator_set[1]
        );
        assert_eq!(
            vec![Operator::Addition, Operator::Addition, Operator::Addition,],
            operator_set[26]
        );
    }
}
