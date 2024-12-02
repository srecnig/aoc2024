pub fn is_report_safe(report: &[i32]) -> bool {
    let direction = get_level_direction(report[0], report[1]);
    for i in 0..(report.len() - 1) {
        if !are_compatible(report[i], report[i + 1], &direction) {
            return false;
        }
    }

    true
}

fn are_compatible(left: i32, right: i32, direction: &LevelDirectionKind) -> bool {
    let difference = match direction {
        LevelDirectionKind::Ascending => right - left,
        LevelDirectionKind::Descending => left - right,
        LevelDirectionKind::Invalid => return false,
    };

    match difference {
        d if d <= 0 => false,
        d if d > 3 => false,
        _ => true,
    }
}

fn get_level_direction(left: i32, right: i32) -> LevelDirectionKind {
    match left.cmp(&right) {
        std::cmp::Ordering::Equal => LevelDirectionKind::Invalid,
        std::cmp::Ordering::Greater => LevelDirectionKind::Descending,
        std::cmp::Ordering::Less => LevelDirectionKind::Ascending,
    }
}

#[derive(PartialEq, Debug)]
enum LevelDirectionKind {
    Ascending,
    Descending,
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_test_for_safety() {
        assert!(is_report_safe(&[7, 6, 4, 2, 1]));
        assert!(!is_report_safe(&[1, 2, 7, 8, 9]));
        assert!(!is_report_safe(&[9, 7, 6, 2, 1]));
        assert!(!is_report_safe(&[1, 3, 2, 4, 5]));
        assert!(!is_report_safe(&[8, 6, 4, 4, 1]));
        assert!(is_report_safe(&[1, 3, 6, 7, 9]));
        assert!(!is_report_safe(&[7, 7, 4, 2, 1]));
        assert!(!is_report_safe(&[19, 21, 24, 27, 28, 28]));
        assert!(!is_report_safe(&[20, 24, 27, 28, 30, 33, 35]));
    }

    #[test]
    fn can_decide_on_direct() {
        assert_eq!(LevelDirectionKind::Invalid, get_level_direction(5, 5));
        assert_eq!(LevelDirectionKind::Descending, get_level_direction(5, 2));
        assert_eq!(LevelDirectionKind::Ascending, get_level_direction(2, 5));
    }

    #[test]
    fn can_check_if_levels_are_compatible() {
        // equal
        assert!(!are_compatible(5, 5, &LevelDirectionKind::Ascending));
        assert!(!are_compatible(5, 5, &LevelDirectionKind::Descending));
        // ascending
        assert!(are_compatible(5, 8, &LevelDirectionKind::Ascending));
        assert!(!are_compatible(5, 9, &LevelDirectionKind::Ascending));
        assert!(!are_compatible(5, 8, &LevelDirectionKind::Descending));
        assert!(!are_compatible(5, 9, &LevelDirectionKind::Descending));
        // descending
        assert!(are_compatible(5, 2, &LevelDirectionKind::Descending));
        assert!(!are_compatible(5, 1, &LevelDirectionKind::Descending));
        assert!(!are_compatible(5, 2, &LevelDirectionKind::Ascending));
        assert!(!are_compatible(5, 1, &LevelDirectionKind::Ascending));
    }
}
