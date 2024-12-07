#[derive(Debug)]
pub struct Equation {
    pub result: i64,
    pub numbers: Vec<i64>,
}

impl Equation {
    pub fn check(&self) -> bool {
        let operator_count = self.numbers.len() - 1;
        let operator_set = OperatorSet::new(operator_count as u32);

        for operators in operator_set {
            if self.verify_operators(&operators) {
                return true;
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
}

struct OperatorSet {
    count: u32,
    curr: i32,
}

impl OperatorSet {
    fn new(count: u32) -> OperatorSet {
        let base: u32 = 2;
        let curr: i32 = (base.pow(count) - 1) as i32;
        OperatorSet { curr, count }
    }
}

impl Iterator for OperatorSet {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr < 0 {
            return None;
        }

        let binary_string = format!("{:0width$b}", self.curr, width = self.count as usize);
        let operator_set: Vec<Operator> = binary_string
            .chars()
            .map(|ch| match ch {
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
        assert!(!bad_equation.check());
        let good_equation = Equation {
            result: 292,
            numbers: vec![11, 6, 16, 20],
        };
        assert!(good_equation.check());
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
    fn can_iterate_operator_set() {
        let operator_set: Vec<Vec<Operator>> = OperatorSet::new(4).collect();
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
}
