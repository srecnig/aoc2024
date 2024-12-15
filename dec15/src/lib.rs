use std::fmt;

#[derive(Debug)]
pub struct Warehouse {
    tiles: Vec<Vec<Tile>>,
    robot_position: (usize, usize),
}

impl Warehouse {
    pub fn new(lines: Vec<&str>) -> Warehouse {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut robot_position: Option<(usize, usize)> = None;

        for (y, line) in lines.iter().enumerate() {
            let mut row: Vec<Tile> = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                let on_tile: OnTile = match ch {
                    'O' => OnTile::Box,
                    '.' => OnTile::Empty,
                    '@' => OnTile::Robot {},
                    '#' => OnTile::Wall,
                    _ => panic!(),
                };
                if on_tile == OnTile::Robot {
                    robot_position = Some((x, y));
                }
                row.push(Tile { on_tile });
            }
            tiles.push(row);
        }
        let robot_position = robot_position.expect("No robot found!");

        Warehouse {
            tiles,
            robot_position,
        }
    }

    pub fn move_robot(&mut self, movement: &Movement) {
        match movement.direction {
            MovementDirection::Up => {
                let x = self.robot_position.0;
                let mut y = self.robot_position.1 - 1;
                let empty_space: Option<(usize, usize)>;
                loop {
                    if self.tiles[y][x].is_wall() {
                        return;
                    }
                    if self.tiles[y][x].is_empty() {
                        empty_space = Some((x, y));
                        break;
                    }
                    y -= 1;
                }

                let mut current_y = empty_space.expect("weird. no empty space, no wall.").1;
                let mut next_y = current_y + 1;
                while next_y < self.robot_position.1 {
                    let moved_from_tile = self.tiles[next_y][x].on_tile;
                    self.tiles[current_y][x].place(moved_from_tile);
                    self.tiles[next_y][x].place(OnTile::Empty);
                    current_y += 1;
                    next_y += 1;
                }
                self.tiles[current_y][x].place(OnTile::Robot);
                self.tiles[next_y][x].place(OnTile::Empty);
                self.robot_position = (x, current_y);
            }
            MovementDirection::Right => {
                let mut x = self.robot_position.0 + 1;
                let y = self.robot_position.1;
                let empty_space: Option<(usize, usize)>;
                loop {
                    if self.tiles[y][x].is_wall() {
                        return;
                    }
                    if self.tiles[y][x].is_empty() {
                        empty_space = Some((x, y));
                        break;
                    }
                    x += 1;
                }

                let mut current_x = empty_space.expect("weird. no empty space, no wall.").0;
                let mut next_x = current_x - 1;
                while next_x > self.robot_position.0 {
                    let moved_from_tile = self.tiles[y][next_x].on_tile;
                    self.tiles[y][current_x].place(moved_from_tile);
                    self.tiles[y][next_x].place(OnTile::Empty);
                    current_x -= 1;
                    next_x -= 1;
                }
                self.tiles[y][current_x].place(OnTile::Robot);
                self.tiles[y][next_x].place(OnTile::Empty);
                self.robot_position = (current_x, y);
            }
            MovementDirection::Down => {
                let x = self.robot_position.0;
                let mut y = self.robot_position.1 + 1;
                let empty_space: Option<(usize, usize)>;
                loop {
                    if self.tiles[y][x].is_wall() {
                        return;
                    }
                    if self.tiles[y][x].is_empty() {
                        empty_space = Some((x, y));
                        break;
                    }
                    y += 1;
                }

                let mut current_y = empty_space.expect("weird. no empty space, no wall.").1;
                let mut next_y = current_y - 1;
                while next_y > self.robot_position.1 {
                    let moved_from_tile = self.tiles[next_y][x].on_tile;
                    self.tiles[current_y][x].place(moved_from_tile);
                    self.tiles[next_y][x].place(OnTile::Empty);
                    current_y -= 1;
                    next_y -= 1;
                }
                self.tiles[current_y][x].place(OnTile::Robot);
                self.tiles[next_y][x].place(OnTile::Empty);
                self.robot_position = (x, current_y);
            }
            MovementDirection::Left => {
                let mut x = self.robot_position.0 - 1;
                let y = self.robot_position.1;
                let empty_space: Option<(usize, usize)>;
                loop {
                    if self.tiles[y][x].is_wall() {
                        return;
                    }
                    if self.tiles[y][x].is_empty() {
                        empty_space = Some((x, y));
                        break;
                    }
                    x -= 1;
                }

                let mut current_x = empty_space.expect("weird. no empty space, no wall.").0;
                let mut next_x = current_x + 1;
                while next_x < self.robot_position.0 {
                    let moved_from_tile = self.tiles[y][next_x].on_tile;
                    self.tiles[y][current_x].place(moved_from_tile);
                    self.tiles[y][next_x].place(OnTile::Empty);
                    current_x += 1;
                    next_x += 1;
                }
                self.tiles[y][current_x].place(OnTile::Robot);
                self.tiles[y][next_x].place(OnTile::Empty);
                self.robot_position = (current_x, y);
            }
        }
    }

    pub fn gps_sum(&self) -> u32 {
        let mut sum = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, _tile) in row.iter().enumerate().filter(|(_, t)| t.is_box()) {
                sum += 100 * y + x;
            }
        }
        sum as u32
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: String = "".to_string();

        for tile_row in &self.tiles {
            let mut row_output: String = "".to_string();
            for tile in tile_row {
                row_output = format!("{}{}", row_output, tile.on_tile);
            }
            output = format!("{}\n{}", output, row_output);
        }

        write!(f, "{}", output)
    }
}

#[derive(Debug)]
struct Tile {
    on_tile: OnTile,
}

impl Tile {
    fn is_empty(&self) -> bool {
        matches!(self.on_tile, OnTile::Empty)
    }

    fn is_box(&self) -> bool {
        matches!(self.on_tile, OnTile::Box)
    }

    fn is_wall(&self) -> bool {
        matches!(self.on_tile, OnTile::Wall)
    }

    fn is_robot(&self) -> bool {
        matches!(self.on_tile, OnTile::Robot)
    }

    fn place(&mut self, on_tile: OnTile) {
        self.on_tile = on_tile;
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum OnTile {
    Box,
    Empty,
    Robot,
    Wall,
}

impl fmt::Display for OnTile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ch: char = match self {
            OnTile::Box => 'O',
            OnTile::Empty => '.',
            OnTile::Robot => '@',
            OnTile::Wall => '#',
        };
        write!(f, "{}", ch)
    }
}

#[derive(Debug)]
pub struct Movement {
    direction: MovementDirection,
}

impl Movement {
    pub fn new(ch: char) -> Movement {
        let direction: MovementDirection = match ch {
            '^' => MovementDirection::Up,
            '>' => MovementDirection::Right,
            'v' => MovementDirection::Down,
            '<' => MovementDirection::Left,
            _ => panic!(),
        };
        Movement { direction }
    }
}

#[derive(PartialEq, Debug)]
enum MovementDirection {
    Up,
    Right,
    Down,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_warehouse() {
        let lines = vec!["######", "##@O.#"];
        let warehouse = Warehouse::new(lines);
        assert_eq!((2, 1), warehouse.robot_position);

        assert_eq!(OnTile::Wall, warehouse.tiles[0][0].on_tile);
        assert_eq!(OnTile::Wall, warehouse.tiles[0][1].on_tile);
        assert_eq!(OnTile::Wall, warehouse.tiles[0][2].on_tile);
        assert_eq!(OnTile::Wall, warehouse.tiles[0][3].on_tile);
        assert_eq!(OnTile::Wall, warehouse.tiles[0][4].on_tile);
        assert_eq!(OnTile::Wall, warehouse.tiles[0][5].on_tile);

        assert_eq!(OnTile::Wall, warehouse.tiles[1][0].on_tile);
        assert_eq!(OnTile::Wall, warehouse.tiles[1][1].on_tile);
        assert_eq!(OnTile::Robot, warehouse.tiles[1][2].on_tile);
        assert_eq!(OnTile::Box, warehouse.tiles[1][3].on_tile);
        assert_eq!(OnTile::Empty, warehouse.tiles[1][4].on_tile);
        assert_eq!(OnTile::Wall, warehouse.tiles[1][5].on_tile);
    }

    #[test]
    fn can_create_direction() {
        let mut movements: Vec<Movement> = Vec::new();
        let input = "<vv>^";
        for ch in input.chars() {
            movements.push(Movement::new(ch));
        }
        assert_eq!(MovementDirection::Left, movements[0].direction);
        assert_eq!(MovementDirection::Down, movements[1].direction);
        assert_eq!(MovementDirection::Down, movements[2].direction);
        assert_eq!(MovementDirection::Right, movements[3].direction);
        assert_eq!(MovementDirection::Up, movements[4].direction);
    }

    #[test]
    fn can_bump_into_wall() {
        // ######
        // ##@O.#
        let lines = vec!["######", "##@O.#"];
        let mut warehouse = Warehouse::new(lines);
        let move_up = Movement::new('^');
        warehouse.move_robot(&move_up);
        assert_eq!(2, warehouse.robot_position.0); // x
        assert_eq!(1, warehouse.robot_position.1); // y
    }

    #[test]
    fn can_move_upwards_with_boxes() {
        // ####
        // #..#
        // #O.#
        // #O.#
        // #..#
        // #@.#
        let lines = vec!["####", "#..#", "#O.#", "#O.#", "#..#", "#@.#"];
        let mut warehouse = Warehouse::new(lines);
        let move_up = Movement::new('^');
        // move up once
        warehouse.move_robot(&move_up);
        assert_eq!(1, warehouse.robot_position.0); // x
        assert_eq!(4, warehouse.robot_position.1); // y

        // one more time, this time also move boxes
        warehouse.move_robot(&move_up);
        assert_eq!(1, warehouse.robot_position.0); // x
        assert_eq!(3, warehouse.robot_position.1); // y
        assert!(warehouse.tiles[0][1].is_wall());
        assert!(warehouse.tiles[1][1].is_box());
        assert!(warehouse.tiles[2][1].is_box());
        assert!(warehouse.tiles[3][1].is_robot());

        // third time, nothing will happen
        warehouse.move_robot(&move_up);
        assert_eq!(1, warehouse.robot_position.0); // x
        assert_eq!(3, warehouse.robot_position.1); // y
        assert!(warehouse.tiles[0][1].is_wall());
        assert!(warehouse.tiles[1][1].is_box());
        assert!(warehouse.tiles[2][1].is_box());
        assert!(warehouse.tiles[3][1].is_robot());
    }

    #[test]
    fn can_calculate_gps_sum() {
        // #######
        // #...O..
        // #@.....
        let lines = vec!["#######", "#...O..", "#@....."];
        let warehouse = Warehouse::new(lines);
        assert_eq!(104, warehouse.gps_sum());
    }
}
