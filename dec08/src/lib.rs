use std::collections::HashMap;

pub struct AntennaMap {
    max_dimension: Point,
    antennas: HashMap<char, Vec<Point>>,
}

impl AntennaMap {
    pub fn new(lines: Vec<&str>) -> AntennaMap {
        let max_dimension = Point {
            x: lines[0].len() as i32 - 1,
            y: lines.len() as i32 - 1,
        };

        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '.' => (),
                    _ => {
                        let point = Point {
                            x: x as i32,
                            y: y as i32,
                        };
                        antennas.entry(ch).or_insert_with(Vec::new).push(point);
                    }
                };
            }
        }

        AntennaMap {
            max_dimension,
            antennas,
        }
    }

    pub fn antinodes(&self) -> Vec<Point> {
        let mut antinodes: Vec<Point> = Vec::new();

        for (_ch, points) in &self.antennas {
            for i in 0..(points.len() - 1) {
                for j in i + 1..points.len() {
                    let (antinode1, antinode2) = Self::calculate_antinodes(&points[i], &points[j]);
                    if !antinodes.contains(&antinode1) {
                        antinodes.push(antinode1);
                    }
                    if !antinodes.contains(&antinode2) {
                        antinodes.push(antinode2);
                    }
                }
            }
        }
        antinodes
            .into_iter()
            .filter(|an| {
                an.x <= self.max_dimension.x
                    && an.y <= self.max_dimension.y
                    && an.x >= 0
                    && an.y >= 0
            })
            .collect()
    }

    fn calculate_antinodes(one: &Point, other: &Point) -> (Point, Point) {
        let x_diff = other.x - one.x;
        let y_diff = other.y - one.y;

        (
            Point {
                x: one.x - x_diff,
                y: one.y - y_diff,
            },
            Point {
                x: other.x + x_diff,
                y: other.y + y_diff,
            },
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_scan_antenna_map() {
        let lines = vec![
            "..........",
            "..........",
            "....a.....",
            "........a.",
            ".....a....",
            "..........",
            "..........",
        ];
        let map = AntennaMap::new(lines);
        assert_eq!(1, map.antennas.len());
        assert_eq!(3, map.antennas.get(&'a').unwrap().len());
    }

    #[test]
    fn can_get_all_antinodes() {
        let lines = vec![
            "..........",
            "..........",
            "....a.....",
            "........a.",
            ".....a....",
            "..........",
            "..........",
        ];
        let map = AntennaMap::new(lines);
        let antinodes = map.antinodes();
        assert_eq!(4, antinodes.len());
        assert!(antinodes.contains(&Point { x: 0, y: 1 }));
        assert!(antinodes.contains(&Point { x: 3, y: 0 }));
        assert!(antinodes.contains(&Point { x: 6, y: 6 }));
        assert!(antinodes.contains(&Point { x: 2, y: 5 }));
    }

    #[test]
    fn can_calcuate_antinodes() {
        let point_a = Point { x: 4, y: 2 };
        let point_b = Point { x: 5, y: 4 };
        let (antinode_a, antinode_b) = AntennaMap::calculate_antinodes(&point_a, &point_b);
        assert_eq!(antinode_a, Point { x: 3, y: 0 });
        assert_eq!(antinode_b, Point { x: 6, y: 6 });

        let point_c = Point { x: 5, y: 4 };
        let point_d = Point { x: 8, y: 3 };
        let (antinode_c, antinode_d) = AntennaMap::calculate_antinodes(&point_c, &point_d);
        assert_eq!(antinode_c, Point { x: 2, y: 5 });
        assert_eq!(antinode_d, Point { x: 11, y: 2 });
    }
}
