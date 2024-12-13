use regex::Regex;

#[derive(Debug)]
pub struct ClawMachine {
    button_a: Button,
    button_b: Button,
    prize_location: Location,
}

#[derive(Debug)]
struct Button {
    x_delta: i128,
    y_delta: i128,
}

#[derive(Debug)]
struct Location {
    x: i128,
    y: i128,
}

pub fn build_machine(lines: Vec<&str>, corrected: bool) -> ClawMachine {
    let re_a = Regex::new(r"Button A: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let button_a_captures = re_a.captures(lines[0]).unwrap();
    let button_a = Button {
        x_delta: button_a_captures["x"].parse().unwrap(),
        y_delta: button_a_captures["y"].parse().unwrap(),
    };

    let re_b = Regex::new(r"Button B: X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
    let button_b_captures = re_b.captures(lines[1]).unwrap();
    let button_b = Button {
        x_delta: button_b_captures["x"].parse().unwrap(),
        y_delta: button_b_captures["y"].parse().unwrap(),
    };

    let re_p = Regex::new(r"Prize: X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
    let prize_captures = re_p.captures(lines[2]).unwrap();
    let mut x_prize: i128 = prize_captures["x"].parse().unwrap();
    let mut y_prize: i128 = prize_captures["y"].parse().unwrap();
    if corrected {
        x_prize += 10000000000000;
        y_prize += 10000000000000;
    }

    let prize_location = Location {
        x: x_prize,
        y: y_prize,
    };

    ClawMachine {
        button_a,
        button_b,
        prize_location,
    }
}

pub fn win_prize(machine: &ClawMachine) -> Option<(i128, i128)> {
    if machine.button_a.x_delta * 100 + machine.button_b.x_delta * 100 < machine.prize_location.x
        || machine.button_a.y_delta * 100 + machine.button_b.y_delta * 100
            < machine.prize_location.y
    {
        return None;
    }
    for press_a in 1..101 {
        for press_b in 1..101 {
            if press_a * machine.button_a.x_delta + press_b * machine.button_b.x_delta
                == machine.prize_location.x
                && press_a * machine.button_a.y_delta + press_b * machine.button_b.y_delta
                    == machine.prize_location.y
            {
                return Some((press_a, press_b));
            }
        }
    }
    None
}

pub fn win_prize_efficiently(machine: &ClawMachine) -> Option<(i128, i128)> {
    let press_b_remainder = (machine.prize_location.y * machine.button_a.x_delta
        - machine.prize_location.x * machine.button_a.y_delta)
        % (machine.button_b.y_delta * machine.button_a.x_delta
            - machine.button_b.x_delta * machine.button_a.y_delta);
    let press_b = (machine.prize_location.y * machine.button_a.x_delta
        - machine.prize_location.x * machine.button_a.y_delta)
        / (machine.button_b.y_delta * machine.button_a.x_delta
            - machine.button_b.x_delta * machine.button_a.y_delta);

    let press_a_remainder =
        (machine.prize_location.x - machine.button_b.x_delta * press_b) % machine.button_a.x_delta;
    let press_a =
        (machine.prize_location.x - machine.button_b.x_delta * press_b) / machine.button_a.x_delta;

    if press_a_remainder != 0 || press_b_remainder != 0 || press_a < 0 || press_b < 0 {
        return None;
    }

    Some((press_a, press_b))
}

pub fn calculate_price(press_a: i128, press_b: i128) -> i128 {
    press_a * 3 + press_b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_machine() {
        let lines = vec![
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
        ];
        let machine = build_machine(lines, false);
        assert_eq!(94, machine.button_a.x_delta);
        assert_eq!(34, machine.button_a.y_delta);
        assert_eq!(22, machine.button_b.x_delta);
        assert_eq!(67, machine.button_b.y_delta);
        assert_eq!(8400, machine.prize_location.x);
        assert_eq!(5400, machine.prize_location.y);
    }

    #[test]
    fn can_build_machine_correctly() {
        let lines = vec![
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
        ];
        let machine = build_machine(lines, true);
        assert_eq!(94, machine.button_a.x_delta);
        assert_eq!(34, machine.button_a.y_delta);
        assert_eq!(22, machine.button_b.x_delta);
        assert_eq!(67, machine.button_b.y_delta);
        assert_eq!(10000000008400, machine.prize_location.x);
        assert_eq!(10000000005400, machine.prize_location.y);
    }

    #[test]
    fn can_abort_if_pointless_on_x() {
        let button_a = Button {
            x_delta: 1,
            y_delta: 34,
        };
        let button_b = Button {
            x_delta: 2,
            y_delta: 67,
        };
        let prize_location = Location { x: 8400, y: 5400 };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!(None, win_prize(&claw_machine));
    }

    #[test]
    fn can_abort_if_pointless_on_y() {
        let button_a = Button {
            x_delta: 94,
            y_delta: 2,
        };
        let button_b = Button {
            x_delta: 22,
            y_delta: 1,
        };
        let prize_location = Location { x: 8400, y: 5400 };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!(None, win_prize(&claw_machine));
    }

    #[test]
    fn can_detect_presses() {
        let button_a = Button {
            x_delta: 94,
            y_delta: 34,
        };
        let button_b = Button {
            x_delta: 22,
            y_delta: 67,
        };
        let prize_location = Location { x: 8400, y: 5400 };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!((80, 40), win_prize(&claw_machine).unwrap());
    }

    #[test]
    fn can_detect_presses_efficiently() {
        let button_a = Button {
            x_delta: 94,
            y_delta: 34,
        };
        let button_b = Button {
            x_delta: 22,
            y_delta: 67,
        };
        let prize_location = Location { x: 8400, y: 5400 };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!((80, 40), win_prize_efficiently(&claw_machine).unwrap());
    }

    #[test]
    fn can_not_find_presses() {
        let button_a = Button {
            x_delta: 26,
            y_delta: 66,
        };
        let button_b = Button {
            x_delta: 67,
            y_delta: 21,
        };
        let prize_location = Location { x: 12748, y: 12176 };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!(None, win_prize(&claw_machine));
    }

    #[test]
    fn can_not_find_presses_efficiently() {
        let button_a = Button {
            x_delta: 26,
            y_delta: 66,
        };
        let button_b = Button {
            x_delta: 67,
            y_delta: 21,
        };
        let prize_location = Location { x: 12748, y: 12176 };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!(None, win_prize_efficiently(&claw_machine));
    }

    #[test]
    fn can_debug() {
        let button_a = Button {
            x_delta: 15,
            y_delta: 49,
        };
        let button_b = Button {
            x_delta: 54,
            y_delta: 27,
        };
        let prize_location = Location { x: 5009, y: 4005 };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!(None, win_prize(&claw_machine));
        assert_eq!(None, win_prize_efficiently(&claw_machine));
    }

    #[test]
    fn can_debug_more() {
        let button_a = Button {
            x_delta: 94,
            y_delta: 34,
        };
        let button_b = Button {
            x_delta: 22,
            y_delta: 67,
        };
        let prize_location = Location {
            x: 10000000008400,
            y: 10000000005400,
        };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!(None, win_prize_efficiently(&claw_machine));
    }

    #[test]
    fn can_debug_even_more() {
        let button_a = Button {
            x_delta: 69,
            y_delta: 23,
        };
        let button_b = Button {
            x_delta: 27,
            y_delta: 71,
        };
        let prize_location = Location {
            x: 10000000018641,
            y: 10000000010279,
        };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!(
            Some((102851800151, 107526881786)),
            win_prize_efficiently(&claw_machine)
        );
    }

    #[test]
    fn can_debug_one_last_time() {
        let button_a = Button {
            x_delta: 5,
            y_delta: 3,
        };
        let button_b = Button {
            x_delta: 5,
            y_delta: 9,
        };
        let prize_location = Location { x: 10, y: 10 };
        let claw_machine = ClawMachine {
            button_a,
            button_b,
            prize_location,
        };
        assert_eq!(None, win_prize_efficiently(&claw_machine));
    }

    #[test]
    fn can_calculate_price() {
        assert_eq!(280, calculate_price(80, 40));
    }
}
