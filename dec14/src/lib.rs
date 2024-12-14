use regex::Regex;

pub struct Map {
    width: i32,
    height: i32,
}

#[derive(Debug)]
pub struct Robot {
    x: i32,
    y: i32,
    x_speed: i32,
    y_speed: i32,
}

impl Map {
    pub fn new(width: i32, height: i32) -> Map {
        Map { width, height }
    }
}

impl Robot {
    pub fn new(definition: &str) -> Robot {
        let re = Regex::new(r"p=(?<x>\d+),(?<y>-?\d+)\s+v=(?<xs>-?\d+),(?<ys>-?\d+)").unwrap();
        let captures = re.captures(definition).unwrap();
        Robot {
            x: captures["x"].parse().unwrap(),
            y: captures["y"].parse().unwrap(),
            x_speed: captures["xs"].parse().unwrap(),
            y_speed: captures["ys"].parse().unwrap(),
        }
    }

    pub fn travel(&mut self, map: &Map) {
        let mut new_x = self.x + self.x_speed;
        if new_x < 0 {
            new_x = map.width - new_x.abs();
        }
        if new_x > map.width - 1 {
            new_x = (map.width - 1 - new_x).abs() - 1;
        }
        self.x = new_x;

        let mut new_y = self.y + self.y_speed;
        if new_y < 0 {
            new_y = map.height - new_y.abs();
        }
        if new_y > map.height - 1 {
            new_y = (map.height - 1 - new_y).abs() - 1;
        }
        self.y = new_y;
    }
}

pub fn count_by_quadrant(robots: &[Robot], map: &Map) -> (i32, i32, i32, i32) {
    let edge_x = map.width / 2;
    let edge_y = map.height / 2;

    let top_left_count = robots
        .iter()
        .filter(|r| r.x < edge_x && r.y < edge_y)
        .count();

    let top_right_count = robots
        .iter()
        .filter(|r| r.x > edge_x && r.y < edge_y)
        .count();

    let bottom_left_count = robots
        .iter()
        .filter(|r| r.x < edge_x && r.y > edge_y)
        .count();

    let bottom_right_count = robots
        .iter()
        .filter(|r| r.x > edge_x && r.y > edge_y)
        .count();

    (
        top_left_count as i32,
        top_right_count as i32,
        bottom_left_count as i32,
        bottom_right_count as i32,
    )
}

pub fn robot_display(robots: &[Robot], map: &Map) -> Vec<Vec<char>> {
    let mut output: Vec<Vec<char>> = vec![vec![' '; map.width as usize]; map.height as usize];

    for (x, y) in robots.iter().map(|r| (r.x, r.y)) {
        output[y as usize][x as usize] = '*';
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_make_robot() {
        // // p=0,4 v=3,-3
        let robot = Robot::new("p=0,4 v=3,-3");
        assert_eq!(0, robot.x);
        assert_eq!(4, robot.y);
        assert_eq!(3, robot.x_speed);
        assert_eq!(-3, robot.y_speed);
    }

    #[test]
    fn can_make_robot_travel() {
        let mut robot = Robot {
            x: 2,
            y: 4,
            x_speed: 2,
            y_speed: -3,
        };
        let map = Map {
            width: 11,
            height: 7,
        };
        // second 1
        robot.travel(&map);
        assert_eq!(4, robot.x);
        assert_eq!(1, robot.y);
        // second 2
        robot.travel(&map);
        assert_eq!(6, robot.x);
        assert_eq!(5, robot.y);
        // second 3
        robot.travel(&map);
        assert_eq!(8, robot.x);
        assert_eq!(2, robot.y);
        // // second 4
        robot.travel(&map);
        assert_eq!(10, robot.x);
        assert_eq!(6, robot.y);
        // // second 5
        robot.travel(&map);
        assert_eq!(1, robot.x);
        assert_eq!(3, robot.y);
    }

    #[test]
    fn can_count_robots() {
        let map = Map {
            width: 11,
            height: 7,
        };
        let robots: Vec<Robot> = vec![
            Robot {
                x: 6,
                y: 0,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 6,
                y: 0,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 9,
                y: 0,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 0,
                y: 2,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 3,
                y: 5,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 4,
                y: 5,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 4,
                y: 5,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 1,
                y: 6,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 6,
                y: 6,
                x_speed: 0,
                y_speed: 0,
            },
        ];

        let (top_left_count, top_right_count, bottom_left_count, bottom_right_count) =
            count_by_quadrant(&robots, &map);
        assert_eq!(1, top_left_count);
        assert_eq!(3, top_right_count);
        assert_eq!(4, bottom_left_count);
        assert_eq!(1, bottom_right_count);
    }

    #[test]
    fn can_display_robots() {
        let map: Map = Map::new(3, 4);
        let robots: Vec<Robot> = vec![
            Robot {
                x: 0,
                y: 0,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 1,
                y: 1,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 2,
                y: 2,
                x_speed: 0,
                y_speed: 0,
            },
            Robot {
                x: 0,
                y: 3,
                x_speed: 0,
                y_speed: 0,
            },
        ];

        let display: Vec<Vec<char>> = robot_display(&robots, &map);
        assert_eq!(vec!['*', ' ', ' '], display[0]);
        assert_eq!(vec![' ', '*', ' '], display[1]);
        assert_eq!(vec![' ', ' ', '*'], display[2]);
        assert_eq!(vec!['*', ' ', ' '], display[3]);

        println!("{:#?}", display);
    }
}

// pub fn display_robots(robots: &[Robot], map: &Map) -> Vec<Vec<char>> {
