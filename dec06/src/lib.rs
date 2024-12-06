pub fn build_map(lines: Vec<&str>) -> Map {
    let mut points: Vec<Vec<Cell>> = Vec::new();
    let mut player: Option<Player> = None;

    for (y, line) in lines.iter().enumerate() {
        let mut row: Vec<Cell> = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            let cell = match ch {
                '^' => Cell {
                    visited: true,
                    content: CellContent::Player,
                },
                '#' => Cell {
                    visited: false,
                    content: CellContent::Obstacle,
                },
                '.' => Cell {
                    visited: false,
                    content: CellContent::Empty,
                },
                _ => Cell {
                    visited: false,
                    content: CellContent::Empty,
                },
            };
            if ch == '^' {
                player = Some(Player {
                    x,
                    y,
                    direction: PlayerFacing::Up,
                });
            }
            row.push(cell);
        }
        points.push(row);
    }
    let player = player.expect("Player not found in the map");

    Map { points, player }
}

pub struct Map {
    points: Vec<Vec<Cell>>,
    player: Player,
}

impl Map {
    fn at(&self, x: usize, y: usize) -> &Cell {
        &self.points[y][x]
    }

    fn turn_player(&mut self) {
        match &self.player.direction {
            PlayerFacing::Up => {
                self.player.direction = PlayerFacing::Right;
            }
            PlayerFacing::Right => {
                self.player.direction = PlayerFacing::Down;
            }
            PlayerFacing::Down => {
                self.player.direction = PlayerFacing::Left;
            }
            PlayerFacing::Left => {
                self.player.direction = PlayerFacing::Up;
            }
        };
    }

    pub fn move_player(&mut self) -> bool {
        let (next_x, next_y) = match self.player.direction {
            PlayerFacing::Up => {
                if self.player.y == 0 {
                    return false;
                }
                (self.player.x, self.player.y - 1)
            }
            PlayerFacing::Right => {
                if self.player.x == (self.points[0].len() - 1) {
                    return false;
                }
                (self.player.x + 1, self.player.y)
            }
            PlayerFacing::Down => {
                if self.player.y == self.points.len() - 1 {
                    return false;
                }
                (self.player.x, self.player.y + 1)
            }
            PlayerFacing::Left => {
                if self.player.x == 0 {
                    return false;
                }
                (self.player.x - 1, self.player.y)
            }
        };

        let next_cell = &(self.points[next_y][next_x]);
        match next_cell.content {
            CellContent::Empty | CellContent::Player => {
                self.player.y = next_y;
                self.player.x = next_x;
                self.points[self.player.y][self.player.x].visited = true;
            }
            CellContent::Obstacle => {
                self.turn_player();
            }
        }

        true
    }

    pub fn visited_count(&self) -> i32 {
        self.points
            .iter()
            .map(|row| row.iter().filter(|&c| c.visited).count() as i32)
            .sum()
    }
}

struct Cell {
    visited: bool,
    content: CellContent,
}

#[derive(PartialEq, Debug)]
enum CellContent {
    Player,
    Obstacle,
    Empty,
}

struct Player {
    x: usize,
    y: usize,
    direction: PlayerFacing,
}

#[derive(PartialEq, Debug)]
enum PlayerFacing {
    Up,
    Right,
    Down,
    Left,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_map() {
        let lines = vec![".#", "^."];
        let map: Map = build_map(lines);
        assert_eq!(CellContent::Empty, map.at(0, 0).content);
        assert_eq!(CellContent::Obstacle, map.at(1, 0).content);
        assert_eq!(CellContent::Player, map.at(0, 1).content);
        assert_eq!(CellContent::Empty, map.at(1, 1).content);
        assert_eq!(0, map.player.x);
        assert_eq!(1, map.player.y);
    }

    #[test]
    fn can_turn_player() {
        let lines = vec![".#", "^."];
        let mut map = build_map(lines);
        assert_eq!(PlayerFacing::Up, map.player.direction);
        map.turn_player();
        assert_eq!(PlayerFacing::Right, map.player.direction);
        map.turn_player();
        assert_eq!(PlayerFacing::Down, map.player.direction);
        map.turn_player();
        assert_eq!(PlayerFacing::Left, map.player.direction);
        map.turn_player();
        assert_eq!(PlayerFacing::Up, map.player.direction);
    }

    #[test]
    fn can_move_player() {
        let lines = vec![".#", "^."];
        // . #
        // ^ .
        let mut map = build_map(lines);
        // move up once
        assert!(map.move_player());
        assert_eq!(PlayerFacing::Up, map.player.direction);
        assert_eq!(0, map.player.x);
        assert_eq!(0, map.player.y);
        // we're out of bounds and didn't move.
        assert!(!map.move_player());
        assert_eq!(0, map.player.x);
        assert_eq!(0, map.player.y);
        // turn right
        map.turn_player();
        assert_eq!(PlayerFacing::Right, map.player.direction);
        // move into obstacle, we'll be still at the same place, but have turned once more
        assert!(map.move_player());
        assert_eq!(0, map.player.x);
        assert_eq!(0, map.player.y);
        assert_eq!(PlayerFacing::Down, map.player.direction);
        // 0,0 and 0,1 should've been visited
        assert!(map.at(0, 0).visited);
        assert!(map.at(0, 1).visited);
        assert!(!map.at(1, 0).visited);
        assert!(!map.at(1, 1).visited);
        assert_eq!(2, map.visited_count());
    }
}
