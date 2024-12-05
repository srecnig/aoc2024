#[derive(Debug)]
pub struct Rule {
    pub front: i32,
    pub back: i32,
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.front == other.front && self.back == other.back
    }
}

pub fn filter_relevant_rules<'a>(update: &[i32], rules: &'a Vec<Rule>) -> Vec<&'a Rule> {
    let mut relevant_rules: Vec<&'a Rule> = Vec::new();

    for rule in rules {
        if update.contains(&rule.front) && update.contains(&rule.back) {
            relevant_rules.push(rule);
        }
    }

    relevant_rules
}

pub fn validate_update(update: &[i32], rules: &[&Rule]) -> bool {
    let update_len = update.len();
    for i in 0..update_len {
        // check everything on the left
        let page_number = update[i];
        if i > 0 {
            let page_numbers_left: Vec<i32> = rules
                .iter()
                .filter(|&r| r.back == page_number)
                .map(|r| r.front)
                .collect();
            for page_number_left in page_numbers_left {
                if !update[0..i].contains(&page_number_left) {
                    return false;
                }
            }
        }
        // check everything on the right
        if i < update_len - 1 {
            let page_numbers_right: Vec<i32> = rules
                .iter()
                .filter(|&r| r.front == page_number)
                .map(|r| r.back)
                .collect();
            for page_number_right in page_numbers_right {
                if !update[i..update_len].contains(&page_number_right) {
                    return false;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_filter_relevant_rules() {
        let rules = vec![
            Rule {
                front: 47,
                back: 53,
            },
            Rule {
                front: 97,
                back: 13,
            },
            Rule {
                front: 97,
                back: 61,
            },
            Rule {
                front: 97,
                back: 47,
            },
            Rule {
                front: 75,
                back: 29,
            },
        ];
        let update = vec![47, 75, 53, 13, 97, 4, 8, 29];
        let relevant_rules: Vec<&Rule> = filter_relevant_rules(&update, &rules);
        println!("{:?}", relevant_rules);

        assert!(relevant_rules.contains(&&rules[0]));
        assert!(relevant_rules.contains(&&rules[1]));
        assert!(!relevant_rules.contains(&&rules[2]));
        assert!(relevant_rules.contains(&&rules[3]));
        assert!(relevant_rules.contains(&&rules[4]));
    }

    #[test]
    fn can_validate_correct_update() {
        let rules: Vec<&Rule> = vec![
            &Rule {
                front: 75,
                back: 47,
            },
            &Rule {
                front: 75,
                back: 61,
            },
            &Rule {
                front: 75,
                back: 53,
            },
            &Rule {
                front: 75,
                back: 29,
            },
            &Rule {
                front: 47,
                back: 61,
            },
            &Rule {
                front: 47,
                back: 53,
            },
            &Rule {
                front: 47,
                back: 29,
            },
            &Rule {
                front: 75,
                back: 61,
            },
            &Rule {
                front: 47,
                back: 61,
            },
            &Rule {
                front: 61,
                back: 53,
            },
            &Rule {
                front: 61,
                back: 29,
            },
            &Rule {
                front: 53,
                back: 29,
            },
        ];
        let update = vec![75, 47, 61, 53, 29];
        assert!(validate_update(&update, &rules));
    }

    #[test]
    fn can_validate_incorrect_update() {
        let rules = vec![&Rule {
            front: 97,
            back: 75,
        }];
        let update = vec![75, 97, 47, 61, 53];
        assert!(!validate_update(&update, &rules));
    }
}
